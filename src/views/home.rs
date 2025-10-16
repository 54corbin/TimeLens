use chrono::NaiveTime;
use dioxus::prelude::*;

use crate::components::Bubble;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            background_color: "#f0f0f0",

            Bubble { size: 150, text: "Test 2Test 2Test Home aniimation long long onlngl hlahllj ahljaliln".to_string(), time: NaiveTime::parse_from_str("08:11:00", "%H:%M:%S").unwrap(), color: "#50626b22".to_string(), text_color: "#0000008b".to_string() }
            // Bubble { size: 50, name: "Test 1".to_string(), time: "10:00".to_string() }
            Bubble { size: 100,  text: "Test 2Test 2Test 2".to_string(), time: NaiveTime::parse_from_str("11:00", "%H:%M").unwrap(), color: "#ffb0fffd".to_string(), text_color: "#0000008b".to_string() }
            // Bubble { size: 150, name: "Test 3".to_string(), time: "12:00".to_string() }
            // Bubble { size: 200, name: "Test 4".to_string(), time: "13:00".to_string() }
            // Bubble { size: 250, name: "Test 5".to_string(), time: "14:00".to_string() }
            // Bubble { size: 300, name: "Test 6".to_string(), time: "15:00".to_string() }
            // Bubble { size: 350, name: "Test 7Test 8".to_string(), time: "16:00".to_string() }
            // Bubble { size: 350, name: "Test 8Test 8Test 8".to_string(), time: "17:00".to_string() }
        }
    }
}
