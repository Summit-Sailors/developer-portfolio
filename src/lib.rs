mod layout;
pub use layout::Layout;

mod pages;
pub use pages::{ContactPage, HomePage, NotFound, ProjectsPage, StoryPage};

mod router;
pub use router::Route;

use dioxus::prelude::*;
// #[cfg(not(debug_assertions))]
// static MATOMO_SCRIPT: Asset = asset!("/assets/matomo_tag.js", JsAssetOptions::new().into_asset_options());
pub static FAVICON: Asset = asset!("/assets/favicon.png", ImageAssetOptions::new().with_avif());
pub static TAILWIND: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new().with_minify(true));
pub static HEADSHOT: Asset = asset!("/assets/headshot.png", ImageAssetOptions::new().with_avif());
pub static PLACEHOLDER: Asset =
	asset!("/assets/placeholder.svg", ImageAssetOptions::new().with_avif().with_size(ImageSize::Manual { width: 600, height: 350 }));
