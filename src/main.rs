// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;
use tbt_segmentation::{
    evaluate, generate_toml_output_file, get_best_number_skipped_entries, get_tbt_and_trace,
    parse_command_line,
};

fn main() {
    let start = SystemTime::now();
    /*************
     * PARAMETERS
     *************/
    let arguments = parse_command_line();

    /**********************************
     * Get best number skipped entries
     **********************************/
    let (number_skipped_entries, delta_rho_skipped) = get_best_number_skipped_entries(
        &arguments.specification,
        &arguments.logfile,
        arguments.sub_sampling,
    );

    /*******************
     * STARTUP ROUTINES
     *******************/

    let (trace, tbt) = get_tbt_and_trace(
        &arguments.specification,
        &arguments.logfile,
        number_skipped_entries,
        arguments.lazy_evaluation,
        arguments.sub_sampling,
    );

    /*********************
     * Evaluation
     *********************/
    let (best_segmentation, alternative_segmentations) = evaluate(
        tbt,
        trace,
        start,
        arguments.sub_sampling,
        arguments.lazy_evaluation,
        delta_rho_skipped,
        arguments.print_leaf_segments_only,
        arguments.segmentation_setting,
        arguments.debug_console,
    );

    /*********************
     * Create XML file
     *********************/

    if let Some(toml_output_location) = arguments.toml {
        println!("Generating toml file: {}", toml_output_location);
        match generate_toml_output_file(
            toml_output_location,
            best_segmentation,
            alternative_segmentations,
        ) {
            Ok(_) => println!("Successfully generated toml file."),
            Err(_) => println!("Failed to generate toml file."),
        };
    }

    /*********************
     * Finish Execution
     *********************/
    println!(
        "Finished after {} seconds.",
        start.elapsed().unwrap().as_secs()
    );
}
