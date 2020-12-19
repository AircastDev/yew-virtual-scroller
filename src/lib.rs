#![deny(missing_docs)]

//! ![Crates.io](https://img.shields.io/crates/l/yew-virtual-scroller) ![Crates.io](https://img.shields.io/crates/v/yew-virtual-scroller)
//!
//! A Yew component for virtual scrolling / scroll windowing -- Only renders the visible content into the dom.
//!
//! # Example:
//! ```rust
//! struct MyItem { value: usize }
//!
//! impl From<MyItem> for yew::Html {
//!     fn from(item: MyItem) -> Self {
//!         html! {
//!             // Each item must be the same height.
//!             <div key={item.value} style="height: 32px;">
//!                 {format!("Item: {}", item.value)}
//!             </div>
//!         }
//!     }
//! }
//!
//! fn view(&self) -> yew::Html {
//!     // Items is wrapped with an Rc to avoid cloning large lists.
//!     let items = Rc::clone(&self.items);
//!     html! {
//!         <div>
//!             <style>{"
//!                 /* Scroller should be constrained in some way so it can scroll */
//!                 .scroller {
//!                     height: 600px;
//!                 }
//!             "}</style>
//!
//!             <VirtualScroller<MyItem>
//!                 items={items}
//!                 row_height={32.0}
//!                 class=Classes::from("scroller")
//!             />
//!         </div>
//!     }
//! }
//! ```
//!
//! # License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

use std::{
    cmp::{max, min},
    fmt::Debug,
    ops::Range,
    rc::Rc,
};
use web_sys::Element;
use yew::{html, Classes, Component, NodeRef, Properties};
use yew_component_size::{ComponentSize, ComponentSizeObserver};

const WINDOW_STYLES: &str = "will-change:transform;";

/// Yew component for virtual scrolling / scroll windowing
///
/// See the crate documentation for an example and more information.
pub struct VirtualScroller<T>
where
    T: Into<yew::Html> + Clone + PartialEq + Debug + 'static,
{
    /// Component properties
    pub props: Props<T>,

    link: yew::ComponentLink<Self>,
    viewport_ref: NodeRef,
    viewport_height: f64,
    content_window: Option<ContentWindow>,
}

/// VirtualScroller properties
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props<T>
where
    T: Into<yew::Html> + Clone + PartialEq + Debug + 'static,
{
    /// Full list of items. This is within an Rc as the assumption is the list will be large
    /// and so cloning it would be expensive.
    pub items: Rc<Vec<T>>,

    /// Height of each item in pixels.
    pub row_height: f64,

    /// Class(es) to apply to the root container
    #[prop_or_default]
    pub class: Classes,
}

#[doc(hidden)]
pub enum Msg {
    CalculateViewport,
    UpdateViewportHeight(f64),
    CalculateWindowContent,
}

struct ContentWindow {
    start_y: f64,
    visible_range: Range<usize>,
}

impl<T> Component for VirtualScroller<T>
where
    T: Into<yew::Html> + Clone + PartialEq + Debug + 'static,
{
    type Message = Msg;

    type Properties = Props<T>;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            viewport_ref: Default::default(),
            viewport_height: 0f64,
            content_window: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::CalculateViewport => {
                let viewport = self.viewport_ref.cast::<Element>().unwrap();
                self.viewport_height = viewport.client_height() as f64;
                true
            }
            Msg::UpdateViewportHeight(height) => {
                self.viewport_height = height;
                true
            }
            Msg::CalculateWindowContent => {
                let node_padding: usize = 0;
                let viewport = self.viewport_ref.cast::<Element>().unwrap();
                let scroll_top = viewport.scroll_top() as f64;
                let start_node = max(
                    0,
                    ((scroll_top / self.props.row_height).floor() as isize)
                        - (node_padding as isize),
                ) as usize;
                let total_visible = min(
                    ((self.viewport_height / self.props.row_height).ceil()) as usize
                        + 2 * node_padding,
                    self.props.items.len() - start_node,
                );
                let start_y = (start_node as f64) * self.props.row_height;
                let end_node = start_node + total_visible;
                self.content_window = Some(ContentWindow {
                    start_y,
                    visible_range: start_node..end_node,
                });
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            let should_rerender = self.props.class != props.class;
            self.props = props;
            self.link.send_message(Msg::CalculateWindowContent);
            should_rerender
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        let total_content_height = (self.props.items.len() as f64) * self.props.row_height;
        let content_style = format!("height: {}px", total_content_height);

        let (window_style, windowed_items) = match &self.content_window {
            Some(cw) => (
                format!("{}transform: translateY({}px);", WINDOW_STYLES, cw.start_y),
                (&self.props.items[cw.visible_range.clone()]).into(),
            ),
            None => (WINDOW_STYLES.to_string(), vec![]),
        };
        let items = windowed_items.into_iter().map(|item| item.into());

        let onscroll = self.link.callback(|_| Msg::CalculateWindowContent);
        let onsize = self.link.batch_callback(|rect: ComponentSize| {
            vec![
                Msg::UpdateViewportHeight(rect.height),
                Msg::CalculateWindowContent,
            ]
        });

        html! {
            <div ref=self.viewport_ref.clone() onscroll=onscroll class=self.props.class.clone() style="position: relative; overflow: auto">
                <div style=content_style>
                    <div style=window_style>
                        {for items}
                    </div>
                </div>
                <ComponentSizeObserver onsize=onsize />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link
                .send_message_batch(vec![Msg::CalculateViewport, Msg::CalculateWindowContent]);
        }
    }
}
