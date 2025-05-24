use crate::build_order::{
    Action, ActionType, BuildOrder, BuildOrderError, BuildType, Difficulty, OrderEntry, Race,
};
use crate::build_regex::*;
use crate::http_client::HttpClient;
use chrono::NaiveDate;
use regex::Regex;
use std::str::FromStr;

pub const BUILD_URL: &str = "https://lotv.spawningtool.com/build/";

struct HeaderContent {
    name: String,
    player_race: String,
    opponent_race: String,
    build_type: String,
}

struct DetailsContent {
    author: String,
    published: Option<String>,
    patch: String,
    difficulty: Option<String>,
}

/// Extracts the name of the build order from the HTML content.
fn extract_header(html_content: &str) -> HeaderContent {
    let mut content = HeaderContent {
        name: String::new(),
        player_race: String::new(),
        opponent_race: String::new(),
        build_type: String::new(),
    };
    let re = Regex::new(HEADER_REGEX).unwrap();
    let captures = re.captures(html_content).unwrap();
    if let Some(name) = captures.get(1) {
        content.name = name.as_str().to_string();
    }
    if let Some(player_race) = captures.get(2) {
        content.player_race = player_race.as_str().to_string();
    }
    if let Some(opponent_race) = captures.get(3) {
        content.opponent_race = opponent_race.as_str().to_string();
    }
    if let Some(build_type) = captures.get(4) {
        content.build_type = build_type.as_str().to_string();
    }
    content
}

/// Extracts the details of the build order from the HTML content.
fn extract_details(html_content: &str) -> DetailsContent {
    let mut content = DetailsContent {
        author: String::new(),
        published: None,
        patch: String::new(),
        difficulty: None,
    };
    let re = Regex::new(DETAILS_REGEX).unwrap();
    if let Some(captures) = re.captures(html_content) {
        if let Some(author) = captures.get(1) {
            content.author = author.as_str().to_string();
        }
        if let Some(published) = captures.get(2) {
            if !published.as_str().is_empty() {
                content.published = Some(published.as_str().to_string());
            }
        }
        if let Some(patch) = captures.get(3) {
            content.patch = patch.as_str().to_string();
        }
        if let Some(difficulty) = captures.get(4) {
            content.difficulty = Some(difficulty.as_str().to_string());
        }
    }
    content
}

/// Extracts the description of the build order from the HTML content.
fn extract_description(html_content: &str) -> Option<String> {
    let re = Regex::new(DESCRIPTION_REGEX).unwrap();
    re.captures(html_content)
        .and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
}

/// Extracts the VOD from the HTML content.
fn extract_vod(html_content: &str) -> Option<String> {
    let re = Regex::new(VOD_REGEX).unwrap();
    re.captures(html_content)
        .and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
}

/// Extracts the votes from the HTML content.
fn extract_votes(html_content: &str) -> Option<(u32, u32)> {
    let re = Regex::new(VOTES_REGEX).unwrap();
    re.captures(html_content).and_then(|captures| {
        if captures.len() == 3 {
            let percentage = captures.get(1)?.as_str().parse::<u32>().ok()?;
            let votes = captures.get(2)?.as_str().parse::<u32>().ok()?;
            Some((percentage, votes))
        } else {
            None
        }
    })
}

/// Extracts the steps of the build order from the HTML content.
fn extract_steps(html_content: &str) -> Result<Vec<OrderEntry>, BuildOrderError> {
    let mut steps = Vec::new();
    let re = Regex::new(BUILD_TABLE_REGEX).unwrap();
    if let Some(captures) = re.captures(html_content) {
        let table_content = captures.get(1).map_or("", |m| m.as_str());
        let entry_re = Regex::new(BUILD_ENTRY_REGEX).unwrap();
        for entry in entry_re.captures_iter(table_content) {
            if entry.len() < 5 {
                continue; // Skip invalid entries
            }
            let supply: u8 = entry[1].parse().unwrap();
            let time = entry[2].to_string();
            let actions_html = entry[3].to_string();
            let action_re = Regex::new(BUILD_ACTION_REGEX).unwrap();
            let mut actions = Vec::new();
            for action in action_re.captures_iter(&actions_html) {
                if action.len() < 3 {
                    continue; // Skip invalid actions
                }
                let action_type = ActionType::from_str(&action[1]).unwrap();
                let name = action[2].to_string();
                actions.push(Action::new(action_type, name));
            }
            let comment = entry[4].to_string();
            steps.push(OrderEntry::new(supply, time, actions, comment));
        }
    } else {
        return Err(BuildOrderError::ParseError(
            "Failed to find build table in HTML content".to_string(),
        ));
    }
    Ok(steps)
}

impl BuildOrder {
    fn set_header(&mut self, header: HeaderContent) {
        self.set_name(header.name);
        self.set_player_race(Race::from_str(&header.player_race).unwrap());
        self.set_opponent_race(Race::from_str(&header.opponent_race).unwrap());
        self.set_build_type(BuildType::from_str(&header.build_type).unwrap());
    }

    fn set_details(&mut self, details: DetailsContent) {
        self.set_creator(details.author);
        if let Some(published) = details.published {
            if let Ok(date) = NaiveDate::parse_from_str(&published, "%b %d, %Y") {
                self.set_published(date);
            }
        }
        self.set_patch(details.patch);
        if let Some(difficulty) = details.difficulty {
            self.set_difficulty(Difficulty::from_str(&difficulty).unwrap());
        }
    }
}

/// Parses a build order from the given HTML content and returns a `BuildOrder`.
pub fn parse_build_order(html_content: &str) -> Result<BuildOrder, BuildOrderError> {
    let mut build_order = BuildOrder::new();
    let header = extract_header(html_content);
    build_order.set_header(header);

    // Parse the build order description
    if let Some(description) = extract_description(html_content) {
        build_order.set_description(description);
    }

    // Parse the VOD
    if let Some(vod) = extract_vod(html_content) {
        build_order.set_vod(vod);
    }

    // Parse the build order details
    let details = extract_details(html_content);
    build_order.set_details(details);

    // Parse the build order steps
    let steps = extract_steps(html_content)?;
    for step in steps {
        build_order.add_step(step);
    }

    // Parse the votes
    if let Some((percentage, votes)) = extract_votes(html_content) {
        build_order.set_votes(percentage, votes);
    }

    Ok(build_order)
}

pub fn fetch_build_order(build_id: u32) -> Result<BuildOrder, BuildOrderError> {
    let url = format!("{}{}", BUILD_URL, build_id);
    match HttpClient::fetch_url(&url) {
        Ok(response) => {
            if response.status_code != 200 {
                return Err(BuildOrderError::HttpError(
                    "Failed to fetch build order".to_string(),
                ));
            }
            parse_build_order(&response.body)
        }
        Err(e) => Err(BuildOrderError::HttpError(e)),
    }
}
