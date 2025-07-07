use aidoku::{
	alloc::{String, Vec},
	Manga, MangaPageResult, Chapter, Page, MangaStatus, ContentRating, Result,
	imports::net::Request,
};

use crate::helper;

pub fn search_manga(query: Option<String>, page: i32) -> Result<MangaPageResult> {
	let url = format!("https://sito-manga.com/api/search?q={}&page={}", 
		query.clone().unwrap_or_default(), page);

	let html = Request::get(&url).header("User-Agent", "Aidoku").html()?;

	let mut entries: Vec<Manga> = Vec::new();

	for node in html.select(".manga-entry").array() {
		let element = node.as_node().expect("Should be a node");
		let title = element.select("h3.title").text().read();
		let key = element.select("a").attr("href").read();
		let cover = element.select("img").attr("src").read();

		entries.push(Manga {
			title,
			key,
			cover: Some(cover),
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		entries,
		has_next_page: false,
	})
}

pub fn update_manga(
	manga: &mut Manga,
	needs_details: bool,
	needs_chapters: bool,
) -> Result<()> {
	if needs_details {
		let html = Request::get(&manga.key).html()?;

		manga.authors = Some(vec![String::from("Autore")]);
		manga.description = Some(html.select(".desc").text().read());
		manga.status = MangaStatus::Ongoing;
		manga.content_rating = ContentRating::Safe;
		manga.tags = Some(vec![String::from("Azione"), String::from("Fantasy")]);
	}

	if needs_chapters {
		let html = Request::get(&format!("{}/chapters", manga.key)).html()?;

		let mut chapters: Vec<Chapter> = Vec::new();
		for (i, node) in html.select(".chapter").array().enumerate() {
			let element = node.as_node().expect("Chapter must be a node");
			let chapter_title = element.select(".title").text().read();
			let chapter_key = element.select("a").attr("href").read();
			chapters.push(Chapter {
				key: chapter_key,
				title: Some(chapter_title),
				chapter_number: Some(i as f32 + 1.0),
				..Default::default()
			});
		}
		manga.chapters = Some(chapters);
	}

	Ok(())
}

pub fn get_pages(_manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
	let html = Request::get(&chapter.key).html()?;

	let mut pages: Vec<Page> = Vec::new();

	for (i, node) in html.select(".page-img").array().enumerate() {
		let url = node.as_node().expect("Page").attr("src").read();
		pages.push(Page {
			index: i as i32,
			content: aidoku::PageContent::url(url),
			..Default::default()
		});
	}

	Ok(pages)
}
