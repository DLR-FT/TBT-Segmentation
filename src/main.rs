// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;
use tbt_segmentation::{behaviortree::Tbt, *};

/****************************************
   This needs to be defined by the user!
   She needs to provide a Trace and a Tbt
****************************************/
fn user_defined(
    logfile: &str,
    number_skipped_entries: usize,
    lazy_evaluation: bool,
    sub_sampling: bool,
) -> (Trace, Tbt) {
    get_tbt_and_trace(
        logfile,
        number_skipped_entries,
        lazy_evaluation,
        sub_sampling,
    )
}

fn main() {
    let start = SystemTime::now();
    /*************
     * PARAMETERS
     *************/
    let arguments = parse_command_line();

    /**********************************
     * Get best number skipped entries
     **********************************/
    let (number_skipped_entries, delta_rho_skipped) =
        get_best_number_skipped_entries(&arguments.logfile, arguments.sub_sampling);

    /*******************
     * STARTUP ROUTINES
     *******************/
    let (trace, tbt) = user_defined(
        &arguments.logfile,
        number_skipped_entries,
        arguments.lazy_evaluation,
        arguments.sub_sampling,
    );

    /*********************
     * Evaluation
     *********************/
    evaluate(
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
     * Finish Execution
     *********************/
    println!(
        "Finished after {} seconds.",
        start.elapsed().unwrap().as_secs()
    );
}
