# yew-virtual-scroller

![Crates.io](https://img.shields.io/crates/l/yew-virtual-scroller) ![Crates.io](https://img.shields.io/crates/v/yew-virtual-scroller)

A Yew component for virtual scrolling / scroll windowing -- Only renders the visible content into the dom.

## Example:
```rust
struct MyItem { value: usize }

impl From<MyItem> for yew::Html {
    fn from(item: MyItem) -> Self {
        html! {
            // Each item must be the same height.
            <div key={item.value} style="height: 32px;">
                {format!("Item: {}", item.value)}
            </div>
        }
    }
}

fn view(&self) -> yew::Html {
    // Items is wrapped with an Rc to avoid cloning large lists.
    let items = Rc::clone(&self.items);
    html! {
        <div>
            <style>{"
                /* Scroller should be constrained in some way so it can scroll */
                .scroller {
                    height: 600px;
                }
            "}</style>

            <VirtualScroller<MyItem>
                items={items}
                row_height={32.0}
                class=Classes::from("scroller")
            />
        </div>
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
