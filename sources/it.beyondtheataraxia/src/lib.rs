#![no_std]
use aidoku::{Source, Viewer, prelude::*};
use madara::{Impl, Madara, Params};

const BASE_URL: &str = "https://www.beyondtheataraxia.com";

struct Beyond;

impl Impl for Beyond {
    fn new() -> Self {
        Self
    }

    fn params(&self) -> Params {
        Params {
            base_url: BASE_URL.into(),
            source_path: "manga-list".into(),
            use_style_images: true,
            datetime_format: "D MMMM yyyy".into(),
            page_list_selector: "div.reading-content img.wp-manga-chapter-img".into(),
            filter_non_manga_items: true,
            ..Default::default()
        }
    }
}

register_source!(
    Madara<Beyond>,
    DeepLinkHandler,
    MigrationHandler,
    ImageRequestProvider
);
