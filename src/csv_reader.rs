// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{behaviortree::TbtNode, stl::Stl, Trace};
use csv::ReaderBuilder;
use std::fs::File;

pub fn read_csv_file(
    file_name: &str,
    column_name: &str,
    number_skipped_entries: usize,
) -> Vec<f32> {
    let delimiter = b',';
    // Open the CSV file
    let file = File::open(file_name).unwrap();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .from_reader(file);

    let column_index = reader
        .headers()
        .unwrap()
        .iter()
        .position(|header| header == column_name)
        .ok_or(format!("Column '{}' not found in CSV header.", column_name))
        .unwrap();

    // Iterate through the CSV records and print the selected column
    let mut trace_given_name = Vec::<f32>::new();
    let mut take_only_each_tenth_item = if number_skipped_entries == 0 {
        number_skipped_entries
    } else {
        number_skipped_entries - 1
    };
    for result in reader.records() {
        let record = result.unwrap();
        if let Some(str_number) = record.get(column_index) {
            // Attempt to parse the string into a f32
            if number_skipped_entries == 0
                || take_only_each_tenth_item == number_skipped_entries - 1
            {
                take_only_each_tenth_item = 0;
                match str_number.parse::<f32>() {
                    Ok(f) => {
                        trace_given_name.push(f);
                    }
                    Err(e) => {
                        println!("Failed to parse: {}", e);
                    }
                }
            } else {
                take_only_each_tenth_item += 1;
            }
        }
    }
    trace_given_name
}

pub fn get_best_number_skipped(trace: Trace, tree: TbtNode) -> (usize, (f32, f32), (f32, f32)) {
    let atomics = tree.get_atomics();

    let mut global_streak_pos = usize::MAX;
    let mut global_streak_pos_dif = (f32::MAX, f32::MIN);
    let mut global_streak_neg = usize::MAX;
    let mut global_streak_neg_dif = (f32::MAX, f32::MIN);
    for ap in atomics {
        let mut pos_number_consecutive_true = usize::MAX;
        let mut pos_number_consecutive_true_dif = (f32::MAX, f32::MIN);
        let mut pos_interval_values = (f32::MAX, f32::MIN);
        let mut pos_count = 0_usize;
        let mut neg_number_consecutive_false = usize::MAX;
        let mut neg_number_consecutive_false_dif = (f32::MAX, f32::MIN);
        let mut neg_interval_values = (f32::MAX, f32::MIN);
        let mut neg_count = 0_usize;
        for i in 0..trace.0 {
            if let Stl::Atomic(_, names, function) = ap {
                let v = ap.evaluate_fnc(names, &trace, i, function);
                // Positive values
                if v >= 0.0 {
                    pos_count += 1;
                    pos_interval_values.0 = f32::min(pos_interval_values.0, v);
                    pos_interval_values.1 = f32::max(pos_interval_values.1, v);
                } else if pos_count > 0 {
                    if pos_count < pos_number_consecutive_true {
                        pos_number_consecutive_true = pos_count;
                        pos_number_consecutive_true_dif = pos_interval_values;
                    }
                    pos_count = 0;
                }
                // Negative values
                if v < 0.0 {
                    neg_count += 1;
                    neg_interval_values.0 = f32::min(neg_interval_values.0, v);
                    neg_interval_values.1 = f32::max(neg_interval_values.1, v);
                } else if neg_count > 0 {
                    if neg_count < neg_number_consecutive_false {
                        neg_number_consecutive_false = neg_count;
                        neg_number_consecutive_false_dif = pos_interval_values;
                    }
                    neg_count = 0;
                }
            } else {
                panic!("Expected only Atomic propositions here");
            }
        }
        if pos_number_consecutive_true < global_streak_pos {
            global_streak_pos = pos_number_consecutive_true;
            global_streak_pos_dif = pos_number_consecutive_true_dif;
        }
        if neg_number_consecutive_false < global_streak_neg {
            global_streak_neg = neg_number_consecutive_false;
            global_streak_neg_dif = neg_number_consecutive_false_dif;
        }
    }
    println!("Minimal negative streak: {global_streak_neg}, minimal positive streak: {global_streak_pos}");
    if global_streak_pos == usize::MAX || global_streak_neg == usize::MAX {
        (0, global_streak_pos_dif, global_streak_neg_dif)
    } else {
        let mut number_skipped = usize::min(global_streak_pos, global_streak_neg) - 1;
        let frequency = 0.005;
        if number_skipped != 0 {
            while number_skipped > 0 {
                let new_freq = frequency * number_skipped as f32;
                let number_events = 1.0 / new_freq;
                if number_events.fract() == 0.0 {
                    break;
                }
                number_skipped -= 1;
            }
        }
        (number_skipped, global_streak_pos_dif, global_streak_neg_dif)
    }
}
