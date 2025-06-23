#![allow(non_snake_case)] // Allow PascalCase for component names

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod constants;
mod icons;
mod components {
    pub mod layout;
    pub mod ui;
}
mod pages {
    pub mod home;
    pub mod story;
    pub mod projects;
    pub mod contact;
}

use components::layout::{Header, Footer};
use pages::{home::HomePage, story::StoryPage, projects::ProjectsPage, contact::ContactPage};

// Define routes
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    HomePage {},
    #[route("/story")]
    StoryPage {},
    #[route("/projects")]
    ProjectsPage {},
    #[route("/contact")]
    ContactPage {},
    #[route("/:..segments")] // Catch-all for 404 or redirect
    NotFound { segments: Vec<String> },
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Dioxus app starting...");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {
            // The main layout wraps all routed content
            div {
                class: "flex flex-col min-h-screen", // Ensure full height for layout
                Header {},
                main {
                    class: "flex-grow pt-16", // pt-16 for fixed header
                    Outlet::<Route> {} // Where routed content will be rendered
                }
                Footer {},
            }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let route = segments.join("/");
    rsx! {
        div {
            class: "container mx-auto px-4 py-24 text-center",
            h1 { class: "text-4xl font-bold text-purple-500 mb-4", "404 - Page Not Found" }
            p { class: "text-zinc-300", "Sorry, the page you were looking for at '/{route}' does not exist." }
            Link {
                to: Route::HomePage {},
                class: "mt-8 inline-block px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors",
                "Go to Homepage"
            }
        }
    }
}
