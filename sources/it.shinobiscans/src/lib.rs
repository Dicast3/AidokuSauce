#![no_std]
use aidoku::{Source, Viewer, prelude::*};
use madara::LoadMoreStrategy;
use madara::{Impl, Madara, Params};

const BASE_URL: &str = "https://shinobiscans.com";

struct Shin;

impl Impl for Shin {
    fn new() -> Self {
        Self
    }

    fn params(&self) -> Params {
        Params {
            base_url: BASE_URL.into(),
            default_viewer: Viewer::Webtoon,
            datetime_format: "dd MMMM yyyy".into(),
            use_load_more_request: LoadMoreStrategy::Never,
            genre_endpoint: "/manga-genre".into(),
            ..Default::default()
        }
    }
}

register_source!(
    Madara<Shin>,
    DeepLinkHandler,
    MigrationHandler,
    ImageRequestProvider
);
