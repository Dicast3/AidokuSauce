#![no_std]
use aidoku::{prelude::*, Source};
use mangareader::{MangareaderSource, Params};

const BASE_URL: &str = "https://tuttoanimemanga.net";

register_source!(
	MangareaderSource,
	Params {
		base_url: BASE_URL.into(),
		lang: "it".into(), // o "en", "es", dipende
		..Default::default()
	}
);
