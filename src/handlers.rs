use crate::build_order::{BuildOrder, BuildOrderError};
use crate::build_parser::fetch_build_order;
use crate::index_manager::{LOWEST_INDEX, get_st_highest_index};
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn fetch_latest(count: u32) -> Vec<BuildOrder> {
    let spinner_style = ProgressStyle::with_template(
        "[{percent:.bold.dim}%] {elapsed:.dim} {spinner} {bar}\t{wide_msg}",
    )
    .unwrap()
    .tick_chars("⡇⣆⣤⣰⢸⠹⠛⠏ ");
    let pb = ProgressBar::new(count as u64);
    pb.set_style(spinner_style);
    pb.enable_steady_tick(Duration::from_millis(100));

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
                    pb.set_message(format!("Build order {} is cloaked, skipping.", current_id));
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
    pb.finish_with_message(format!(
        "{} {} build orders fetched.",
        style("✔").green(),
        build_orders.len()
    ));
    build_orders
}

pub fn fetch_segment(start: u32, end: u32) -> Vec<BuildOrder> {
    let spinner_style = ProgressStyle::with_template(
        "[{percent:.bold.dim}%] {elapsed:.dim} {spinner} {bar}\t{wide_msg}",
    )
    .unwrap()
    .tick_chars("⡇⣆⣤⣰⢸⠹⠛⠏ ");
    let pb = ProgressBar::new((end - start + 1) as u64);
    pb.set_style(spinner_style);

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

    let m = MultiProgress::new();

    let mut build_orders = Vec::new();
    let mut handles = Vec::new();
    let count = (end - start + 1) as usize;

    for id in start..=end {
        let pb_clone = m.add(ProgressBar::new(1));
        let handle = thread::spawn(move || {
            let spinner_style =
                ProgressStyle::with_template("[{prefix:.bold.dim}] {spinner} \t{wide_msg}")
                    .unwrap()
                    .tick_chars("⡇⣆⣤⣰⢸⠹⠛⠏ ");
            pb_clone.set_message(format!("Fetching build order {}", id));
            pb_clone.set_style(spinner_style.clone());
            pb_clone.enable_steady_tick(Duration::from_millis(100));
            pb_clone.set_prefix(format!("{}/{}", id - start + 1, count));
            match fetch_build_order(id) {
                Ok(build_order) => {
                    pb_clone.inc(1);
                    Some(build_order)
                }
                Err(e) => {
                    if e.eq(&BuildOrderError::Cloaked) {
                        pb_clone.set_message(format!("Build order {} is cloaked, skipping.", id));
                    } else {
                        eprintln!("Error fetching build order {}: {}", id, e);
                    }
                    None
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(Some(build_order)) = handle.join() {
            build_orders.push(build_order);
        }
    }

    pb.finish_with_message(format!(
        "{} {} build orders fetched.",
        style("✔").green(),
        build_orders.len()
    ));

    build_orders
}
