use std::rc::Rc;
use yew::prelude::*;
use yew_virtual_scroller::VirtualScroller;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Example {
    items: Rc<Vec<ExampleItem>>,
}

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            items: Rc::new((1..20000).map(ExampleItem::new).collect()),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let items = Rc::clone(&self.items);
        html! {
            <div>
                <style>{"
                    .scroller {
                        height: 600px;
                    }

                    .list-item {
                        height: 32px;
                    }
                "}</style>

                <h1>{"Yew Virtual Scroller Example"}</h1>
                <VirtualScroller<ExampleItem> items={items} row_height={32.0} class=Classes::from("scroller") />
            </div>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ExampleItem {
    value: usize,
}

impl ExampleItem {
    pub fn new(value: usize) -> Self {
        ExampleItem { value }
    }
}

impl From<ExampleItem> for yew::Html {
    fn from(item: ExampleItem) -> Self {
        html! {
            <div key={item.value} class="list-item">
                {format!("Item: {}", item.value)}
            </div>
        }
    }
}

pub fn main() {
    App::<Example>::new().mount_to_body();
}
