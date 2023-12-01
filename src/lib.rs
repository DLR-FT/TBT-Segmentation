// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

mod tree {
    pub mod shipdeck_landing {
        pub mod get_trace_and_tree;
        pub mod atomics {
            pub mod combined;
            pub mod constants;
            pub mod descend_touchdown;
            pub mod heading_aligned;
            pub mod heading_obliqued;
            pub mod in_position;
            pub mod move_to_touchdown;
            pub mod velocity_aligned;
        }
        pub mod deg45_maneuver;
        pub mod lateral_maneuver;
        pub mod oblique_maneuver;
        pub mod straight_maneuver;
    }
}

pub mod behaviortree;
mod command_line_parser;
mod csv_reader;
mod stl;
mod table;
#[cfg(test)]
mod tests;
use behaviortree::print_segmentation;
use behaviortree::Segmentation;
use behaviortree::Tbt;
use command_line_parser::CommandLineArguments;
use command_line_parser::SegmentationSetting;
use csv_reader::get_best_number_skipped;
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;
use stl::Stl;
use table::Table;
use tree::shipdeck_landing::get_trace_and_tree::get_trace;
use tree::shipdeck_landing::get_trace_and_tree::get_tree;

// A trace consists of the length of the trace (usize) and a hashmap
// that maps a variable name (String) to its stream of values (Vec<f32>)
pub type Trace = (usize, HashMap<String, Vec<f32>>);

// A function that represents an atomic proposition
pub type ApF = Rc<dyn Fn(&[f32]) -> f32>;

/************************
 * Command Line Arguments
 ************************/
pub fn parse_command_line() -> CommandLineArguments {
    command_line_parser::parse_command_line()
}

/**********************************
 * Returns TBT and Trace
 **********************************/
pub fn get_tbt_and_trace(
    logfile: &str,
    number_skipped_entries: usize,
    lazy_evaluation: bool,
    sub_sampling: bool,
) -> (Trace, Tbt) {
    let trace = get_trace(logfile, number_skipped_entries);
    let tbt = get_tree(number_skipped_entries);
    println!(
        "\nSETTING:\n\tIs greedy: (depth first= {lazy_evaluation}, skip= {sub_sampling}({number_skipped_entries}))\n\tTrace length: {}\n\tNumber nodes: {}\n\tNumber formulas: {}\n\nTemporal behavior tree:\n{}\n",
        trace.0,
        Tbt::get_number_nodes(),
        Stl::get_number_formulas(),
        tbt.tree.pretty_print(true, 2),
    );
    (trace, tbt)
}

/**********************************
 * Get best number skipped entries
 **********************************/
pub fn get_best_number_skipped_entries(logfile: &str, sub_sampling: bool) -> (usize, f32) {
    let trace = get_trace(logfile, 0);
    let tree = get_tree(0).tree;
    let (number_skipped_entries, delta_rho_skipped) = if sub_sampling {
        let (number_skipped_entries, (interval_min, interval_max), (_, _)) =
            get_best_number_skipped(trace, tree);
        let delta_rho = interval_max - interval_min;
        println!("Analysis of how many events can be skipped:\n\tcan be skipped: {number_skipped_entries}\n\trobustness diff {} ({interval_min},{interval_max})", delta_rho);
        (number_skipped_entries, delta_rho)
    } else {
        (0, 0.0)
    };
    (number_skipped_entries, delta_rho_skipped)
}

/***************
 *  Evaluation
 ***************/
#[allow(clippy::too_many_arguments)]
pub fn evaluate(
    tbt: Tbt,
    trace: Trace,
    start: SystemTime,
    sub_sampling: bool,
    lazy_evaluation: bool,
    delta_rho_skipped: f32,
    print_leaf_segments_only: bool,
    segmentation_setting: Option<SegmentationSetting>,
    debug: bool,
) -> f32 {
    // MEMORY ALLOCATIONS
    let mut tree_table = Table::new(Tbt::get_number_nodes(), trace.0);
    println!(
        "Created tree table with {} entries.",
        tree_table.total_entries.to_formatted_string(&Locale::en)
    );
    let mut formula_table = Table::new(Stl::get_number_formulas(), trace.0);
    println!(
        "Created formula table with {} entries.\n",
        formula_table.total_entries.to_formatted_string(&Locale::en)
    );

    let mut depth_manager_tree = HashMap::new();
    // EVALUATION
    let robustness_res = tbt.tree.evaluate(
        &mut depth_manager_tree,
        &mut tree_table,
        &mut formula_table,
        &trace,
        0,
        trace.0 - 1,
        &start,
        debug,
        lazy_evaluation,
    );
    let robustness_res = if lazy_evaluation && robustness_res < 0.0 {
        f32::NEG_INFINITY
    } else {
        robustness_res
    };
    // SEGMENTATION
    let (segmentation, robustness_value) = get_segmentation(
        robustness_res,
        &mut tree_table,
        &mut formula_table,
        start,
        &tbt,
        &trace,
        lazy_evaluation,
        sub_sampling,
        delta_rho_skipped,
        print_leaf_segments_only,
    );

    if !lazy_evaluation {
        if let Some(segmentation_setting) = segmentation_setting {
            get_alternative_segmentation(
                &tbt,
                &mut tree_table,
                &mut formula_table,
                &trace,
                &segmentation,
                robustness_value,
                print_leaf_segments_only,
                segmentation_setting,
            );
        }
    }
    robustness_res
}

/***********************
 * SEGMENTATION
 ***********************/
#[allow(clippy::too_many_arguments)]
fn get_segmentation<'a>(
    robustness_res: f32,
    tree_table: &mut Table,
    formula_table: &mut Table,
    start: SystemTime,
    tbt: &'a Tbt,
    trace: &Trace,
    lazy_evaluation: bool,
    sub_sampling: bool,
    delta_rho_skipped: f32,
    print_children_only: bool,
) -> (Segmentation<'a>, f32) {
    println!(
        "\nStatistics: Robustness value is {} with {} total tree lookups and {} formula lookups\nGet segmentation after {} seconds.",
        robustness_res, tree_table.total_lookups.to_formatted_string(&Locale::en), formula_table.total_lookups.to_formatted_string(&Locale::en),start.elapsed().unwrap().as_secs()
    );

    let segmentation = tbt.tree.get_segmentation(
        tree_table,
        formula_table,
        trace,
        0,
        trace.0 - 1,
        lazy_evaluation,
    );
    /*******************
     * PRINTING RESULTS
     *******************/
    let (robustness_value, segmentation_str) =
        print_segmentation(&segmentation, print_children_only, lazy_evaluation);
    println!(
     "{} segmentation with robustness {robustness_value} and skipping delta of {delta_rho_skipped} is:\n{segmentation_str}", if lazy_evaluation || sub_sampling {"Approximate"} else {"Best"});
    (segmentation, robustness_value)
}

/***************************
 * ALTERNATIVE SEGMENTATION
 ***************************/
#[allow(clippy::too_many_arguments)]
pub fn get_alternative_segmentation(
    tbt: &Tbt,
    tree_table: &mut Table,
    formula_table: &mut Table,
    trace: &Trace,
    segmentation: &Segmentation,
    robustness_value: f32,
    print_leaf_segments_only: bool,
    segmentation_setting: SegmentationSetting,
) {
    let _other_segmentations = tbt.tree.get_alternative_segmentation(
        tree_table,
        formula_table,
        trace,
        0,
        trace.0 - 1,
        segmentation,
        segmentation_setting.tau_dif,
        robustness_value - segmentation_setting.rho_dif,
        segmentation_setting.amount,
        print_leaf_segments_only,
    );
}