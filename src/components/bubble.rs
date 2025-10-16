use chrono::NaiveTime;
use dioxus::prelude::*;

const STYLES: &str = r#"
    .water-toggle {
        display: inline-block;
        position: relative;
    }

    .container {
        position: relative;
        z-index: 1;
        filter: url(#still-water);
        animation: wave 10s infinite ease-in-out;
        transition: 1s;
        display: block;
    }

    .container.checked {
        filter: url(#water-rotate);
        transition: 0.5s;
        animation: wave-fast 8s infinite ease-in-out;
    }

    .water-button {
        position: relative;
        z-index: 5;
        font-size: 1rem;
        padding: 0;
        border: none;
        border-radius: 50%;
        background: #50626b22;
        outline: 3px solid #3f545f33;
        color: #0000008b;
        text-shadow: 0px 0px 8px #79a5adda;
        font-weight: bold;
        backdrop-filter: url(#water) blur(0.4rem);
        cursor: pointer;
        font-family: Tahoma, sans-serif;
        transition: 1s;
        box-shadow: inset 2px 3px 4px #d4f7fe25;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        white-space: pre-line;
        text-align: center;
    }

    .container.checked .water-button {
        text-shadow: 0px 0px 12px #49e1ffda;
        background: #13b0ff3d;
        outline-color: #00aaff00;
        box-shadow:
            inset 0px 0px 8px #7ae9ff25,
            0px 0px 64px #39d9f96f;
    }


    @keyframes wave {
        0%, 100% { 
            transform: scale(1);
        }
        50% { 
            transform: scale(1.2);
        }
    }

    @keyframes wave-fast {
        0%, 100% { 
            transform: scale(1);
        }
        50% { 
            transform: scale(1.2);
        }
    }
"#;

#[derive(Props, PartialEq, Clone)]
pub struct BubbleProps {
    size: i32,
    text: String,
    time: NaiveTime,
    color: String,
    text_color: String,
}

#[component]
pub fn Bubble(props: BubbleProps) -> Element {
    let mut is_checked = use_signal(|| false);
    let formatted_time = props.time.format("%H:%M").to_string();

    rsx! {
        style { {STYLES} }
        div {
            class: "inline-block relative",

            label {
                class: "water-toggle",

                input {
                    r#type: "checkbox",
                    id: "toggle-wave",
                    class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer z-30",
                    checked: is_checked(),
                    oninput: move |evt| {
                        is_checked.set(evt.checked());
                    }
                }

                div {
                    class: if is_checked() { "container checked" } else { "container" },

                    button {
                        class: "water-button",
                        style: "width: {props.size}px; height: {props.size}px; background-color: {props.color}; color: {props.text_color};",
                        "{props.text}"
                        br {}
                        "{formatted_time}"
                    }

                    svg {
                        style: "position: absolute; width: 0; height: 0;",

                        filter {
                            id: "water-rotate",
                            feTurbulence {
                                result: "turbulence",
                                seed: "2",
                                num_octaves: "2",
                                base_frequency: "0.0075 0.0075",
                                r#type: "turbulence"
                            }
                            feComponentTransfer {
                                feFuncR {
                                    r#type: "table",
                                    animateTransform {
                                        repeat_count: "indefinite",
                                        dur: "10s",
                                        values: "0.01;0.02;0.01",
                                        attribute_name: "transform"
                                    }
                                }
                            }
                            feDisplacementMap {
                                y_channel_selector: "g",
                                x_channel_selector: "r",
                                scale: "35",
                                in2: "turbulence",
                                "in": "SourceGraphic"
                            }
                        }

                        filter {
                            id: "still-water",
                            feTurbulence {
                                result: "turbulence",
                                seed: "2",
                                num_octaves: "2",
                                base_frequency: "0.0075 0.0075",
                                r#type: "turbulence"
                            }
                            feComponentTransfer {
                                feFuncR {
                                    r#type: "table",
                                    animateTransform {
                                        repeat_count: "indefinite",
                                        dur: "10s",
                                        values: "0.01;0.02;0.01",
                                        attribute_name: "transform"
                                    }
                                }
                            }
                            feDisplacementMap {
                                y_channel_selector: "g",
                                x_channel_selector: "r",
                                scale: "25",
                                in2: "turbulence",
                                "in": "SourceGraphic"
                            }
                        }
                    }
                }
            }
        }
    }
}
