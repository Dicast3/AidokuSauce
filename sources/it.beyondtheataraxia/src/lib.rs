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
            datetime_locale: "it_IT_POSIX".into(),
            datetime_timezone: "current".into(),
            datetime_format: "dd MMMM yyyy".into(),
            default_viewer: Viewer::RightToLeft,
            page_list_selector: "div.reading-content img.wp-manga-chapter-img".into(),
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
