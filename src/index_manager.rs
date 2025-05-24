use crate::http_client::HttpClient;
use onig::Regex;

/// Lowest index that points to an available build order
const LOWEST_INDEX: u32 = 5;
/// URL to the newest builds page
pub static BUILDS_URL: &str = "https://lotv.spawningtool.com/build/?name=&contributor=&sort_by=r&build_type=&difficulty=&patch=&mine=&fav=&is_tl=";
/// Regex to find the first build index in the HTML response
const FIRST_BUILD_REGEX: &str = r#"<tbody><tr><td><a href="/build/(\d+)/">"#;

/// Retrieves the highest build index from Spawning Tool
pub fn get_st_highest_index() -> u32 {
    get_highest_index(BUILDS_URL)
}

/// Retrieves the highest build index from a given URL
pub fn get_highest_index(url: &str) -> u32 {
    match HttpClient::fetch_url(url) {
        Ok(response) => {
            let re = Regex::new(FIRST_BUILD_REGEX).unwrap();
            if let Some(captures) = re.captures(&response.body) {
                captures
                    .at(1)
                    .map_or(LOWEST_INDEX, |m| m.parse::<u32>().unwrap_or(LOWEST_INDEX))
            } else {
                LOWEST_INDEX
            }
        }
        Err(_) => LOWEST_INDEX,
    }
}
