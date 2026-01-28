use dioxus::prelude::*;

#[component]
pub fn MoonIcon(
    #[props(default = 24)] size: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-moon-icon lucide-moon",
            fill: "none",
            height: "{size}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401" }
        }
    }
}

#[component]
pub fn SunIcon(
    #[props(default = 24)] size: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-sun-icon lucide-sun",
            fill: "none",
            height: "{size}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2" }
            path { d: "M12 20v2" }
            path { d: "m4.93 4.93 1.41 1.41" }
            path { d: "m17.66 17.66 1.41 1.41" }
            path { d: "M2 12h2" }
            path { d: "M20 12h2" }
            path { d: "m6.34 17.66-1.41 1.41" }
            path { d: "m19.07 4.93-1.41 1.41" }
        }
    }
}
