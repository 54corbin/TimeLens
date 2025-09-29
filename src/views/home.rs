use dioxus::prelude::*;

use crate::components::Bubble;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {  
            // background_color: "#6f006f",

            Bubble {size:59}
            Bubble {size:291}
        }
    }
}
