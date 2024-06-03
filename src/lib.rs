// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

mod user_defined {
    pub mod get_trace;
}
pub mod behaviortree;
pub mod command_line_parser;
mod csv_reader;
mod functions;
mod parser;
pub mod stl;
mod table;
mod toml_out;
use behaviortree::print_segmentation;
use behaviortree::tbt_node_reset_count;
use behaviortree::ClonedSegmentation;
use behaviortree::Segmentation;
use behaviortree::Tbt;
use behaviortree::TbtNode;
use command_line_parser::CommandLineArguments;
use command_line_parser::SegmentationSetting;
use csv_reader::get_best_number_skipped;
use num_format::{Locale, ToFormattedString};
use parser::parse;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::time::SystemTime;
use stl::stl_reset_count;
use stl::Stl;
use table::Table;
use toml_out::generate_toml;

/*
 * This trait must be implemented by the user.
 * It is needed to provide a trace and a tree.
 */
struct UserProvidedFunction;

trait ProvidesTraceAndTree {
    fn get_events_per_second(number_skipped_entries: usize) -> u64;
    fn get_trace(logfile: &str, number_skipped_entries: usize) -> Trace;
}

/// A trace consists of the length of the trace (usize) and a hashmap
/// that maps a variable name (String) to its stream of values (Vec<f32>)
pub type Trace = (usize, HashMap<String, Vec<f32>>);

/// A function that represents an atomic proposition
pub type ApF = Rc<dyn Fn(&[f32]) -> f32>;

/************************
 * Command Line Arguments
 ************************/
pub fn parse_command_line() -> CommandLineArguments {
    command_line_parser::parse_command_line()
}

/**********************************
 * TBT Parser
 **********************************/
/// # Arguments
/// * `specification` - Location of specification
/// * `number_skipped_entries` - used for subsampling ie number of entries in the logfile that can be skipped
pub fn parse_tbt(
    specification: &str,
    number_skipped_entries: usize,
    use_prints: bool,
) -> Option<TbtNode> {
    let events_per_seconds = UserProvidedFunction::get_events_per_second(number_skipped_entries);
    let formula = fs::read_to_string(specification).unwrap();
    tbt_node_reset_count();
    stl_reset_count();
    match parse(formula, events_per_seconds.try_into().unwrap(), use_prints) {
        Ok(tbt) => Some(tbt),
        Err(e) => {
            println!("{e}");
            None
        }
    }
}

/**********************************
 * Returns TBT and Trace
 **********************************/
/// # Arguments
/// * `specification` - Location of specification
/// * `logfile` - Location of logfile
/// * `number_skipped_entries` - used for subsampling ie number of entries in the logfile that can be skipped
/// * `lazy_evaluation` - enables/disables lazy evaluation
/// * `sub_sampling` - enables/disables sub sampling
pub fn get_tbt_and_trace(
    specification: &str,
    logfile: &str,
    number_skipped_entries: usize,
    lazy_evaluation: bool,
    sub_sampling: bool,
) -> (Trace, Tbt) {
    let trace = UserProvidedFunction::get_trace(logfile, number_skipped_entries);
    let tbt = Tbt::new(parse_tbt(specification, number_skipped_entries, true).unwrap());
    println!(
        "SETTING:\n\tLogfile: {logfile}\n\tApproximations: lazy evaluation={lazy_evaluation}, subsampling={sub_sampling}(delta: {number_skipped_entries})\n\tTrace length: {}\n\nTemporal behavior tree:\n{}\n",
        trace.0,
        tbt.tree.pretty_print(true, 2),
    );
    (trace, tbt)
}

/*******************************************************
 * Get best number skipped entries by analyzing logfile
 *******************************************************/
pub fn get_best_number_skipped_entries(
    specification: &str,
    logfile: &str,
    sub_sampling: bool,
) -> (usize, f32) {
    let trace = UserProvidedFunction::get_trace(logfile, 0);
    let tree = parse_tbt(specification, 0, false).unwrap();
    let (number_skipped_entries, delta_rho_skipped) = if sub_sampling {
        let (number_skipped_entries, (interval_min, interval_max), (_, _)) =
            get_best_number_skipped(trace, tree);
        let delta_rho = interval_max - interval_min;
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
/// Core function that evaluates a logfile given a TBT specification
/// # Arguments
/// * `tbt` - TBT specification
/// * `trace` - Provided trace that is analyzed
/// * `start` - Used for profiling
/// * `sub_sampling` - Enables/disables sub sampling
/// * `lazy_evaluation` - Enables/disables lazy evaluation
/// * `delta_rho_skipped` - Used for subsampling
/// * `print_leaf_segments_only` - Used for debugging: if true only leaves are printed
/// * `segmentation_setting` - Represents the command line arguments to compute the alternative segmentations
/// * `debug` - Used for progress bar
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
) -> (ClonedSegmentation, Option<Vec<ClonedSegmentation>>) {
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

    let alternative_segmentations = if !lazy_evaluation {
        segmentation_setting.map(|seg| {
            get_alternative_segmentation(
                &tbt,
                &mut tree_table,
                &mut formula_table,
                &trace,
                &segmentation,
                robustness_value,
                print_leaf_segments_only,
                seg,
            )
        })
    } else {
        None
    };
    let segmentation = segmentation
        .into_iter()
        .map(|(node, lower, upper, robustness)| (node.clone(), lower, upper, robustness))
        .collect();
    ((robustness_res, segmentation), alternative_segmentations)
}

/***********************
 * SEGMENTATION
 ***********************/
#[allow(clippy::too_many_arguments)]
/// Used to produce the segmentation
/// # Arguments
/// * `robustness_res` - Robustness result found using evaluate() that is used to find the segmentation here
/// * `tree_table` - TBT table used for dynamic programming
/// * `formula_table` - STL table used for dynamic programming
/// * `start` - Used for profiling
/// * `tbt` - TBT specification
/// * `trace` - Trace that is used
/// * `lazy_evaluation` - Enables/disables lazy evaluation
/// * `sub_sampling` - Enables/disables sub sampling
/// * `delta_rho_skipped` - Delta used for the sub sampling
/// * `print_children_only` - Enables/disables debugging prints
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
     "{} segmentation with robustness {robustness_value} and subsampling delta of {delta_rho_skipped} is:\n{segmentation_str}", if lazy_evaluation || sub_sampling {"Approximate"} else {"Best"});
    (segmentation, robustness_value)
}

/***************************
 * ALTERNATIVE SEGMENTATION
 ***************************/
#[allow(clippy::too_many_arguments)]
/// Provides alternative segmentation using the read command line arguments
/// # Arguments
/// * `tbt` - TBT specification
/// * `tree_table` - TBT table used for dynamic programming
/// * `formula_table` - STL table used for dynamic programming
/// * `trace` - Trace that is used
/// * `segmentation` - Optimal segmenation returned by get_segmentation()
/// * `robustness_value` - Robustness values produced by evaluate()
/// * `print_leaf_segments_only` - Enables/disables to print only leaf nodes
/// * `segmentation_setting` - Read command line arguments such as tau and rho
pub fn get_alternative_segmentation(
    tbt: &Tbt,
    tree_table: &mut Table,
    formula_table: &mut Table,
    trace: &Trace,
    segmentation: &Segmentation,
    robustness_value: f32,
    print_leaf_segments_only: bool,
    segmentation_setting: SegmentationSetting,
) -> Vec<ClonedSegmentation> {
    let other_segmentations = tbt.tree.get_alternative_segmentation(
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
    other_segmentations
        .into_iter()
        .map(|(rho, (_, v))| {
            (
                rho,
                v.into_iter()
                    .map(|(node, lower, upper, robustness)| {
                        (node.clone(), lower, upper, robustness)
                    })
                    .collect(),
            )
        })
        .collect()
}

pub fn generate_toml_output_file(
    location: String,
    best_segmentation: ClonedSegmentation,
    alternative_segmentation: Option<Vec<ClonedSegmentation>>,
) -> std::io::Result<()> {
    generate_toml(location, best_segmentation, alternative_segmentation)
}
