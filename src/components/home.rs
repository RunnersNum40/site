use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Ted Pinkerton" }
    }
}
