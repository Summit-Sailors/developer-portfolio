// src/icons.rs
#![allow(non_snake_case)]
use dioxus::prelude::*;

macro_rules! icon_component {
    ($name:ident, $svg_path:literal) => {
        #[component]
        pub fn $name(size: Option<u32>, class: Option<String>) -> Element {
            let s = size.unwrap_or(24);
            let base_class = "inline-block";
            let combined_class = class.map_or(base_class.to_string(), |c| format!("{} {}", base_class, c));
            rsx! {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "{s}",
                    height: "{s}",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "{combined_class}",
                    path { d: $svg_path }
                }
            }
        }
    };
    ($name:ident, $svg_path1:literal, $svg_path2:literal) => {
        #[component]
        pub fn $name(size: Option<u32>, class: Option<String>) -> Element {
            let s = size.unwrap_or(24);
            let base_class = "inline-block";
            let combined_class = class.map_or(base_class.to_string(), |c| format!("{} {}", base_class, c));
            rsx! {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "{s}",
                    height: "{s}",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "{combined_class}",
                    path { d: $svg_path1 }
                    path { d: $svg_path2 }
                }
            }
        }
    };
     ($name:ident, $svg_path1:literal, $svg_path2:literal, $svg_path3:literal) => {
        #[component]
        pub fn $name(size: Option<u32>, class: Option<String>) -> Element {
            let s = size.unwrap_or(24);
            let base_class = "inline-block";
            let combined_class = class.map_or(base_class.to_string(), |c| format!("{} {}", base_class, c));
            rsx! {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "{s}",
                    height: "{s}",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    class: "{combined_class}",
                    path { d: $svg_path1 }
                    path { d: $svg_path2 }
                    path { d: $svg_path3 }
                }
            }
        }
    };
}

// Feather Icons
icon_component!(FiGithub, "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22");
icon_component!(FiTwitter, "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z");
icon_component!(FiLinkedin, "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z", "M2 9h4v12H2z", "M4 6a2 2 0 1 0 0-4 2 2 0 0 0 0 4z");
icon_component!(FiMail, "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z", "M22 6l-10 7L2 6");
icon_component!(FiArrowDown, "M12 5v14", "M19 12l-7 7-7-7");
icon_component!(FiExternalLink, "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6", "M15 3h6v6", "M10 14L21 3");
icon_component!(FiPhone, "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z");
icon_component!(FiMapPin, "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z", "M12 10a3 3 0 1 0 0-6 3 3 0 0 0 0 6z");
icon_component!(FiMenu, "M3 12h18", "M3 6h18", "M3 18h18");
icon_component!(FiX, "M18 6L6 18", "M6 6l12 12");


// Simple Icons - these often have more complex paths or multiple colors.
// For simplicity, using single path versions if available or focusing on the main shape.
// Actual SVGs from simple-icons might need more careful conversion.
// Placeholder: Using Feather-like simple paths for demonstration.
// You'd replace these with actual SVG paths from Simple Icons.
icon_component!(SiReact, "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"); // Simplified
icon_component!(SiNextdotjs, "M9.16 10.997H7.527V7.522h1.633v3.475zm2.298 3.475h1.633V7.522h-1.633v6.95zm2.445-3.475H16.2V7.522h-2.292v3.475zM21 12c0 4.968-4.032 9-9 9s-9-4.032-9-9 4.032-9 9-9 9 4.032 9 9zm-9 7c3.86 0 7-3.14 7-7s-3.14-7-7-7-7 3.14-7 7 3.14 7 7 7z"); // Simplified
icon_component!(SiTailwindcss, "M12 1.718L2.05 6.972v10.11l9.95 5.254 9.95-5.254V6.972L12 1.718zm0 2.07l7.95 4.208-3.05 1.62-4.9-2.598-4.9 2.598-3.05-1.62L12 3.788zM4.05 8.182l3.05 1.62-3.05 1.62V8.182zm4.9 2.598l3.05 1.62-3.05 1.62-3.05-1.62 3.05-1.62zm6.1 0l3.05 1.62-3.05 1.62V10.78zm-1.1 4.208l-4.9 2.598v-5.196l4.9-2.598v5.196zm-5.05-2.598l-4.9 2.598v-5.196l4.9-2.598v5.196z"); // Simplified
icon_component!(SiTypescript, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiNodedotjs, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiPython, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiRust, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiSvelte, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiFastapi, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(SiNestjs, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified
icon_component!(DiDojo, "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z"); // Simplified placeholder
