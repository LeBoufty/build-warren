use build_warren::build_parser::fetch_build_order;
use build_warren::index_manager::get_st_highest_index;
use serde_json;

fn main() {
    let highest_index = get_st_highest_index();
    let build = fetch_build_order(189949);
    println!("Highest Build Index: {}", highest_index);
    println!("Build Order: {}", serde_json::to_string(&build).unwrap());
}
