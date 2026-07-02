#![no_std]
use aidoku::{prelude::*, Source, Viewer};
use madara::{Impl, Madara, Params};

const BASE_URL: &str = "https://manga.ramaorientalfansub.live/";

struct Rama;

impl Impl for Rama {
    fn new() -> Self {
        Self
    }

    fn params(&self) -> Params {
        Params {
            base_url: BASE_URL.into(),
            source_path: "read".into(),
            datetime_format: "MMMM DD, YYYY".into(),
            chapter_date_selector: "span.chapter-release-date i".into(),
            ..Default::default()
        }
    }
}

register_source!(
    Madara<Rama>,
    DeepLinkHandler,
    MigrationHandler,
    ImageRequestProvider
);
