use crate::build_order::{BuildOrder, BuildOrderError};
use crate::build_parser::fetch_build_order;
use crate::index_manager::{LOWEST_INDEX, get_st_highest_index};

pub fn fetch_latest(count: u32) -> Vec<BuildOrder> {
    let highest_index = get_st_highest_index();
    let mut end_index = if count > highest_index - LOWEST_INDEX {
        LOWEST_INDEX
    } else {
        highest_index - count + 1
    };
    let mut increment = 0;
    let mut current_id = highest_index;
    let mut build_orders = Vec::new();
    while current_id >= end_index && build_orders.len() < count as usize {
        current_id = highest_index - increment;
        match fetch_build_order(current_id) {
            Ok(build_order) => {
                build_orders.push(build_order);
                increment += 1;
            }
            Err(e) => {
                if e.eq(&BuildOrderError::Cloaked) {
                    eprintln!("Build order {} is cloaked, skipping.", current_id);
                    end_index = if end_index > LOWEST_INDEX {
                        end_index - 1
                    } else {
                        LOWEST_INDEX
                    } // Decrease end_index to compensate for the skipped build
                } else {
                    eprintln!("Error fetching build order {}: {}", current_id, e);
                }
                increment += 1; // Increment to avoid infinite loop
                continue; // Some builds might not be available, continue fetching
            }
        }
    }
    build_orders
}

pub fn fetch_segment(start: u32, end: u32) -> Vec<BuildOrder> {
    let highest_index = get_st_highest_index();
    let start = if start < LOWEST_INDEX {
        LOWEST_INDEX
    } else {
        start
    };
    let end = if end > highest_index {
        highest_index
    } else {
        end
    };

    if start > end {
        eprintln!(
            "Start index {} is greater than end index {}. Returning empty segment.",
            start, end
        );
        return Vec::new();
    }

    let mut build_orders = Vec::new();
    for id in (start..=end).rev() {
        match fetch_build_order(id) {
            Ok(build_order) => build_orders.push(build_order),
            Err(e) => eprintln!("Error fetching build order {}: {}", id, e),
        }
    }
    build_orders
}
