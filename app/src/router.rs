use super::ContactPage;
use super::HomePage;
use super::Layout;
use super::NotFound;
use super::ProjectsPage;
use super::StoryPage;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
	#[layout(Layout)]
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
