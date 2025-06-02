// Regex patterns to parse the actual build order
pub const BUILD_TABLE_REGEX: &str = r#"<table id="build-1" class="build-table" cellpadding="0" cellspacing="0">(.*)<\/table><a class="overwolf-link""#;
pub const BUILD_ENTRY_REGEX: &str = r#"<tr><td>&nbsp;&nbsp;(\d{1,3})<\/td><td>&nbsp;&nbsp;(\d?\:?\d{1,2}\:\d{1,2}|)<\/td><td>&nbsp;&nbsp;<nobr>(.*?)<\/nobr><\/td><td>&nbsp;&nbsp;([^<]*)<\/td><\/tr>"#;
pub const BUILD_ACTION_REGEX: &str = r#"<span class="([^"]+)">([^<]+)<\/span>"#;

// Regex pattern to parse the title, races and category of the build order
pub const HEADER_REGEX: &str = r#"<div class="page-header "><h1>(.+) \((T|P|Z)v(T|P|Z|X) (Cheese|All-In|Timing Attack|Economic|Co-op|None)\)<\/h1>"#;

// Regex pattern to parse the build order's details
pub const DETAILS_REGEX: &str = r#"<h4>Details<\/h4><ul>(?:<li>Created by: ([^\s]+) ? ?<\/li>)(?:<li>Published on: ([^<]+|) ? ?<\/li>)(?:<li>Modified on: (?:[^<]+|)<\/li>)(?:<li>Patch: ([^<]+|)<\/li>)(?:<li>Difficulty: ([^<]+|)<\/li>)?"#;

// Regex pattern to parse the build order's VOD
pub const VOD_REGEX: &str = r#"<h3 id="vod-header">VOD<\/h3><a href="([^"]+)" target="#;

// Regex pattern to parse the build order's votes
pub const VOTES_REGEX: &str = r#"<span>Votes&nbsp;<\/span>(?:<small>No votes<\/small>|<span class="[^"]+">(\d{1,3})%<\/span>&nbsp;<small>(\d+) votes<\/small>)"#;

// Regex pattern to parse the build order's description
pub const DESCRIPTION_REGEX: &str =
    r#"<h3 id="description-header">Description<\/h3><p>([^<]*)<\/p>"#;
