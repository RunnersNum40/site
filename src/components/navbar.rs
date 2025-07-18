use crate::Route;
use dioxus::prelude::*;

#[component]
fn Spacer() -> Element {
    rsx! {
        div { class: "spacer" }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Spacer {}
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
