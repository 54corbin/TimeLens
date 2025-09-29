use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BubbleProps {
    /// Size of the SVG (width and height in pixels)
    #[props(default = 100)]
    pub size: u32,
}

#[component]
pub fn Bubble(props: BubbleProps) -> Element {
    let size_str = props.size.to_string();
    let mut selected = use_signal(|| false);
    let glow = selected();

    rsx! {
        svg {
            width: "{size_str}",
            height: "{size_str}",
            view_box: "0 0 10 10",
            onclick: move |_| selected.toggle(),

            defs {
                radialGradient {
                    id: "myGradient",
                    cx: "50%",
                    cy: "50%",
                    fr: "15%",
                    r: "50%",

                    stop {
                        offset: "0%",
                        stop_color: "#6fff6f",
                        stop_opacity: "0.0"
                    }
                    stop {
                        offset: "100%",
                        stop_color: "#6fff6f",
                        stop_opacity: "0.9"
                    }
                }
                if glow {
                    radialGradient {
                        id: "glowGradient",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        fr:"15%",
                        stop {
                            offset: "0%",
                            stop_color: "#6fff6f",
                            stop_opacity: "0"
                        }
                        stop {
                            offset: "15%",
                            stop_color: "#6fff6f",
                            stop_opacity: "1"
                        }
                        stop {
                            offset: "100%",
                            stop_color: "#6fff6f",
                            stop_opacity: "0.19"
                        }
                    }
                    filter {
                        id: "blurFilter",
                        feGaussianBlur {
                            _in: "SourceGraphic",
                            std_deviation: "0.3"
                        }
                    }
                }
            }

            if glow {
                circle {
                    cx: "5",
                    cy: "5",
                    r: "5",
                    fill: "url(#glowGradient)",
                    filter: "url(#blurFilter)",
                    animate {
                        attribute_name: "r",
                        values: "4.5;5;4.5",
                        dur: "4s",
                        repeat_count: "indefinite"
                    }
                }
            }

            circle {
                cx: "5",
                cy: "5",
                r: "2",
                fill: "url(#myGradient)",
                animate {
                    attribute_name: "r",
                    values: "2;2.1;2",
                    dur: "4s",
                    repeat_count: "indefinite"
                }
            }
        }
    }
}
