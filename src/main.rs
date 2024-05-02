// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use std::{fs, time::SystemTime};
use tbt_segmentation::{
    evaluate, get_best_number_skipped_entries, get_tbt_and_trace, parse_command_line, parse_tbt,
};

fn main() {
    let start = SystemTime::now();
    /*************
     * PARAMETERS
     *************/
    // let arguments = parse_command_line();

    /**********************************
     * Get best number skipped entries
     **********************************/
    // let (number_skipped_entries, delta_rho_skipped) =
    // get_best_number_skipped_entries(&arguments.logfile, arguments.sub_sampling);
    let number_skipped_entries = 2;
    let frequency = if number_skipped_entries != 0 {
        0.005 * number_skipped_entries as f32
    } else {
        0.005
    };
    let events_per_second = (1.0 / frequency) as usize;
    println!("events: {events_per_second}");
    /*******************
     * STARTUP ROUTINES
     *******************/
    // let formula =
    //     "? (<2..10> f(a)[ a+ 1], [0..1] f(a)[ a+ 1], f(a)[a+1] IU(2..3) f(b)[ b-1])".to_string();
    // let formula = "a(a)[!sin(a) + !degToRad(a)]".to_string();
    let formula = fs::read_to_string("shiplanding_formula.txt").unwrap();
    parse_tbt(formula, events_per_second);
    todo!("Finished");

    // let (trace, tbt) = get_tbt_and_trace(
    //     &arguments.logfile,
    //     number_skipped_entries,
    //     arguments.lazy_evaluation,
    //     arguments.sub_sampling,
    // );

    /*********************
     * Evaluation
     *********************/
    // evaluate(
    //     tbt,
    //     trace,
    //     start,
    //     arguments.sub_sampling,
    //     arguments.lazy_evaluation,
    //     delta_rho_skipped,
    //     arguments.print_leaf_segments_only,
    //     arguments.segmentation_setting,
    //     arguments.debug_console,
    // );

    // /*********************
    //  * Finish Execution
    //  *********************/
    // println!(
    //     "Finished after {} seconds.",
    //     start.elapsed().unwrap().as_secs()
    // );
}
