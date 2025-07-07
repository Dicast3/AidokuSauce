use aidoku::alloc::String;

/// Rimuove spazi extra e caratteri inutili
pub fn clean_string(input: &str) -> String {
	input.trim().replace("\n", "").into()
}

/// Costruisce un URL con una pagina
pub fn build_url(base: &str, page: i32) -> String {
	format!("{}/page/{}", base, page)
}
