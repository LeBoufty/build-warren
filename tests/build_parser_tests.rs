use build_warren::build_order::{BuildType, Difficulty, Race};
use build_warren::build_parser::parse_build_order;

fn open_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).expect("Failed to read file")
}

#[test]
fn test_parse_clemvsmaxpax() {
    let file_name = "tests/examples/clemvsmaxpax.html";
    let html_content = open_file(file_name);
    assert!(!html_content.is_empty(), "HTML content should not be empty");
    let build_order = parse_build_order(&html_content).expect("Failed to parse build order");

    assert_eq!(build_order.get_name(), "Clem 3reapers 2hellions TvP");
    assert_eq!(build_order.get_player_race(), &Race::Terran);
    assert_eq!(build_order.get_opponent_race(), &Race::Protoss);
    assert_eq!(build_order.get_build_type(), &BuildType::Economic);
    assert_eq!(build_order.get_creator(), "herkoss");
    assert_eq!(build_order.get_patch(), "5.0.11");
    assert_eq!(build_order.get_difficulty(), Some(&Difficulty::Hard));
    assert_eq!(
        build_order.get_description(),
        Some("Active TvP build order, very strong")
    );
    assert_eq!(
        build_order.get_vod(),
        Some("https://youtu.be/J4srkfE-oYs?t=1422")
    );
    assert_eq!(build_order.get_votes().unwrap().get_count(), 3);
    assert_eq!(build_order.get_votes().unwrap().get_score(), 66);
    assert_eq!(
        build_order.get_published().unwrap().to_string(),
        "2025-05-01"
    );
    assert_eq!(build_order.get_entries().len(), 132);
}

#[test]
fn test_parse_oldestbuild() {
    let file_name = "tests/examples/oldestbuild.html";
    let html_content = open_file(file_name);
    assert!(!html_content.is_empty(), "HTML content should not be empty");
    let build_order = parse_build_order(&html_content).expect("Failed to parse build order");

    assert_eq!(build_order.get_name(), "Pseudorandom (ZvP (2))");
    assert_eq!(build_order.get_player_race(), &Race::Zerg);
    assert_eq!(build_order.get_opponent_race(), &Race::Protoss);
    assert_eq!(build_order.get_build_type(), &BuildType::None);
    assert_eq!(build_order.get_creator(), "None");
    assert_eq!(build_order.get_patch(), "3.8.0");
    assert_eq!(build_order.get_difficulty(), None);
    assert_eq!(build_order.get_description(), None);
    assert_eq!(build_order.get_vod(), None);
    assert!(build_order.get_votes().is_none());
    assert!(build_order.get_published().is_none());
    assert_eq!(build_order.get_entries().len(), 107);
}
