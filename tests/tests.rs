// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

pub mod tree {
    pub mod shipdeck_landing {
        pub mod deg45_maneuver;
        pub mod lateral_maneuver;
        pub mod oblique_maneuver;
        pub mod straight_maneuver;
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
    }
}

use tree::shipdeck_landing::atomics::descend_touchdown::descend_touchdown;
use tree::shipdeck_landing::deg45_maneuver::get_45deg_maneuver;
use tree::shipdeck_landing::lateral_maneuver::get_lateral_maneuver;
use tree::shipdeck_landing::oblique_maneuver::get_oblique_maneuver;
use tree::shipdeck_landing::straight_maneuver::get_straight_maneuver;

use std::{collections::HashMap, rc::Rc, time::SystemTime};

use tbt_segmentation::{
    behaviortree::{tbt_node_reset_count, Tbt, TbtNode},
    command_line_parser::SegmentationSetting,
    evaluate, get_best_number_skipped_entries, get_tbt_and_trace,
    stl::{stl_reset_count, Stl},
    Trace,
};

fn run_test(
    traces_with_expected_value: Vec<(Vec<f32>, f32)>,
    signal_name: String,
    tbt: Tbt,
) -> Result<(), String> {
    for (trace, expected) in traces_with_expected_value {
        let trace: Trace = (trace.len(), HashMap::from([(signal_name.clone(), trace)]));
        let robustness = evaluate(
            tbt.clone(),
            trace,
            SystemTime::now(),
            false,
            false,
            0.0,
            false,
            None,
            false,
        );
        if robustness.0 == expected {
            continue;
        } else {
            return Err(format!("Expected {expected} but was {}.", robustness.0));
        }
    }
    Ok(())
}

fn test_hardcoded_maneuver(specification: &str, logfile: &str) {
    let (number_skipped_entries, delta_rho_skipped) =
        get_best_number_skipped_entries(specification, logfile, true);

    /*******************
     * STARTUP ROUTINES
     *******************/

    let (trace, new_tbt) =
        get_tbt_and_trace(specification, logfile, number_skipped_entries, false, true);
    let old_tbt = get_tree(number_skipped_entries);

    /*********************
     * Evaluation
     *********************/
    let setting = Some(SegmentationSetting {
        tau_dif: 20000,
        rho_dif: 50.0,
        amount: 3,
    });
    let trace_2 = trace.clone();
    let setting_2 = setting.clone();
    let start = SystemTime::now();
    println!("Run using parsed tbt!");
    let first_run = evaluate(
        new_tbt,
        trace,
        start,
        true,
        false,
        delta_rho_skipped,
        true,
        setting,
        false,
    );
    println!("Run using hand-coded tbt!");
    let second_run = evaluate(
        old_tbt,
        trace_2,
        start,
        true,
        false,
        delta_rho_skipped,
        true,
        setting_2,
        false,
    );

    assert!(
        first_run.0 == second_run.0,
        "Expected {} but was {} in {}.",
        first_run.0,
        second_run.0,
        logfile
    );
    for i in 0..first_run.1.len() {
        let v1 = &first_run.1[i];
        let v2 = &second_run.1[i];
        assert!(
            v1.1 == v2.1,
            "Entry {i} is expected to have same lower but wasnt {} != {}",
            v1.1,
            v2.1
        );
        assert!(
            v1.2 == v2.2,
            "Entry {i} is expected to have same upper but wasnt {} != {}",
            v1.2,
            v2.2
        );
        assert!(
            v1.3 == v2.3,
            "Entry {i} is expected to have same robustness but wasnt {} != {}",
            v1.3,
            v2.3
        );
    }
    let alternatives_new = first_run.2.unwrap();
    let alternatives_old = second_run.2.unwrap();
    for j in 0..alternatives_new.len() {
        for i in 0..first_run.1.len() {
            let v1 = &alternatives_new[j][i];
            let v2 = &alternatives_old[j][i];
            assert!(
                v1.1 == v2.1,
                "Entry of alternatives i={i} j={j} is expected to have same lower but wasnt {} != {}",
                v1.1,
                v2.1
            );
            assert!(
                v1.2 == v2.2,
                "Entry of alternatives i={i} j={j} is expected to have same upper but wasnt {} != {}",
                v1.2,
                v2.2
            );
            assert!(
                v1.3 == v2.3,
                "Entry of alternatives i={i} j={j} is expected to have same robustness but wasnt {} != {}",
                v1.3,
                v2.3
            );
        }
    }
}

fn get_tree(number_skipped_entries: usize) -> Tbt {
    tbt_node_reset_count();
    stl_reset_count();
    let ship_x = "ship_x".to_string();
    let ship_y = "ship_y".to_string();
    let ship_z = "ship_z".to_string();
    let ship_u = "ship_u".to_string();
    let ship_v = "ship_v".to_string();
    let ship_w = "ship_w".to_string();
    let ship_heading = "ship_heading".to_string();
    let uas_x = "uas_x".to_string();
    let uas_y = "uas_y".to_string();
    let uas_z = "uas_z".to_string();
    let uas_u = "uas_u".to_string();
    let uas_v = "uas_v".to_string();
    let uas_w = "uas_w".to_string();
    let uas_heading = "uas_heading".to_string();

    let frequency = if number_skipped_entries != 0 {
        0.005 * number_skipped_entries as f32
    } else {
        0.005
    };
    let events_per_second = (1.0 / frequency) as u64;

    /*
       Get Maneuvers
    */
    let lateral_maneuver = get_lateral_maneuver(
        &events_per_second,
        &uas_x,
        &uas_y,
        &uas_z,
        &uas_u,
        &uas_v,
        &uas_w,
        &uas_heading,
        &ship_x,
        &ship_y,
        &ship_z,
        &ship_u,
        &ship_v,
        &ship_w,
        &ship_heading,
    );

    let straight_maneuver = get_straight_maneuver(
        &events_per_second,
        &uas_x,
        &uas_y,
        &uas_z,
        &uas_u,
        &uas_v,
        &uas_w,
        &uas_heading,
        &ship_x,
        &ship_y,
        &ship_z,
        &ship_u,
        &ship_v,
        &ship_w,
        &ship_heading,
    );

    let oblique_maneuver = get_oblique_maneuver(
        &events_per_second,
        &uas_x,
        &uas_y,
        &uas_z,
        &uas_u,
        &uas_v,
        &uas_w,
        &uas_heading,
        &ship_x,
        &ship_y,
        &ship_z,
        &ship_u,
        &ship_v,
        &ship_w,
        &ship_heading,
    );

    let deg45_maneuver = get_45deg_maneuver(
        &events_per_second,
        &uas_x,
        &uas_y,
        &uas_z,
        &uas_u,
        &uas_v,
        &uas_w,
        &uas_heading,
        &ship_x,
        &ship_y,
        &ship_z,
        &ship_u,
        &ship_v,
        &ship_w,
        &ship_heading,
    );
    /*
        Build tree
    */
    let maneuvers = TbtNode::fallback(vec![
        lateral_maneuver,
        straight_maneuver,
        oblique_maneuver,
        deg45_maneuver,
    ]);

    let descend = TbtNode::leaf(
        Stl::eventually(Stl::atomic(
            vec![uas_x, uas_y, uas_z, ship_x, ship_y, ship_z],
            Rc::new(descend_touchdown),
        )),
        String::from("descend"),
    );
    let tbt_tree = TbtNode::sequence(maneuvers, descend);
    Tbt::new(tbt_tree)
}

/**
 *
 * Actual Tests
 *
 **/

#[test]
fn test_globally() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            -1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 5.0,
            ],
            3.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            -5.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally(Stl::atomic(
            vec![signal_name.clone()],
            Rc::new(|a: &[f32]| a[0]),
        )),
        String::from("globally"),
    ));
    run_test(traces_with_expected_value, signal_name, tbt).unwrap();
}

#[test]
fn test_globally_interval() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            -1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 5.0,
            ],
            3.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            -5.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            0,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            -1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 2.0, 14.0, 5.0,
            ],
            2.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, -4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            -4.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            3,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 2.0, 14.0, 5.0,
            ],
            3.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 1.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, -4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            -4.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, 4.0, 5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], 4.0),
        (vec![2.0, 3.0, 3.0, -4.0], -4.0),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            3,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            1.0,
        ),
        (
            vec![
                5.0, 22.0, 2.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 2.0, 14.0, 5.0,
            ],
            2.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 2.0, 1.0, 0.0, -21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            0.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -10.0,
        ),
        (
            vec![
                -5.0, 22.0, 3.0, -4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            -5.0,
        ),
        (
            vec![
                -5.0, 22.0, -13.0, 4.0, 5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -13.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], 2.0),
        (vec![2.0, 3.0, 3.0, -4.0], -4.0),
        (vec![2.0, 3.0], 2.0),
        (vec![2.0, -3.0], -3.0),
        (vec![-2.0, -3.0], -3.0),
        (vec![-2.0, 3.0], -2.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            0,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                -5.0, 22.0, -13.0, 4.0, -5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -5.0,
        ),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            4,
            4,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
        f32::NEG_INFINITY,
    )];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::globally_interval(
            4,
            3,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("globally_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name, tbt).unwrap();
}

#[test]
fn test_eventually() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 65.0,
            ],
            65.0,
        ),
        (
            vec![
                55.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            55.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -3.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -1.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -2.0,
            ],
            -2.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually(Stl::atomic(
            vec![signal_name.clone()],
            Rc::new(|a: &[f32]| a[0]),
        )),
        String::from("eventually"),
    ));
    run_test(traces_with_expected_value, signal_name, tbt).unwrap();
}

#[test]
fn test_eventually_interval() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 65.0,
            ],
            65.0,
        ),
        (
            vec![
                55.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            55.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -3.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -1.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -2.0,
            ],
            -2.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            0,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 65.0,
            ],
            65.0,
        ),
        (
            vec![
                51.0, 22.0, 3.0, 4.0, 55.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            55.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -1.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -1.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -3.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -2.0,
            ],
            -2.0,
        ),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            3,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                5.0, 22.0, 3.0, 10.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 65.0,
            ],
            10.0,
        ),
        (
            vec![
                55.0, 22.0, 3.0, 4.0, 5.0, 30.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            30.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, -1.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -1.0,
        ),
        (
            vec![
                -5.0, -22.0, -3.0, -4.0, 1.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            1.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -3.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -3.0, -5.0, -4.0, -21.0, -11.0, -12.0, -3.0, -14.0, -2.0,
            ],
            -3.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], 4.0),
        (vec![2.0, 3.0, 3.0, -4.0], -4.0),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            3,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                15.0, 1.0, 3.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 65.0,
            ],
            15.0,
        ),
        (
            vec![
                55.0, 22.0, 333.0, 4.0, 5.0, 3.0, 21.0, 11.0, 32.0, 3.0, 14.0, 1.0,
            ],
            333.0,
        ),
        (
            vec![
                -5.0, -22.0, -5.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -3.0,
        ),
        (
            vec![
                -1.0, -22.0, -3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -10.0,
            ],
            -1.0,
        ),
        (
            vec![
                -5.0, -22.0, 3.0, -4.0, -5.0, -3.0, -21.0, -11.0, -12.0, -3.0, -14.0, -2.0,
            ],
            3.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], 4.0),
        (vec![2.0, 3.0, 3.0, -4.0], 3.0),
        (vec![5.0, 3.0, 3.0, -4.0], 5.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            0,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            5.0,
        ),
        (
            vec![
                -5.0, 22.0, -13.0, 4.0, -5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            -5.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            4,
            4,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
        f32::NEG_INFINITY,
    )];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually_interval(
            4,
            3,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        ),
        String::from("eventually_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name, tbt).unwrap();
}

#[test]
fn test_until() {
    // Defines traces to test
    let signal_name = "a".to_string();
    let traces_with_expected_value = vec![
        (
            vec![
                1.0, 3.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.5,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, 2.0, 3.0, 4.0, -500000.0,
            ],
            1.5,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, -1.0],
            1.0,
        ),
        (
            vec![-2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            2.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 3.5, 2.0, 3.0, 4.0, 3.0],
            -2.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 10.0],
            -1.5,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until(
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until"),
    ));
    run_test(traces_with_expected_value, signal_name, tbt).unwrap();
}

#[test]
fn test_until_interval() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![
                1.0, 3.0, 3.0, 4.0, 5.0, 1.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.5,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, 2.0, 3.0, 4.0, -500000.0,
            ],
            1.5,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, -1.0],
            1.0,
        ),
        (
            vec![-2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            2.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 3.5, 2.0, 3.0, 4.0, 3.0],
            -2.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 10.0],
            -1.5,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            0,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![
                1.0, 3.0, 3.0, 4.0, 5.0, 0.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            0.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 0.0, 5.0, 3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            0.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.5,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, 2.0, 3.0, 4.0, -500000.0,
            ],
            1.5,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, -1.0],
            1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, -3.0],
            1.5,
        ),
        (
            vec![-1.0, 3.0, 3.0, -2.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            2.0,
        ),
        (
            vec![1.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 3.5, 2.0, 3.0, 4.0, 3.0],
            -2.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 10.0],
            -1.5,
        ),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            3,
            usize::MAX,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![
                1.0, 3.0, 3.0, 4.0, 5.0, 0.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            0.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, -3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            2.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 0.0, 5.0, -1.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            0.0,
        ),
        (
            vec![
                2.0, 3.0, 1.0, 4.0, 5.0, -3.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 1.0, -3.0, 2.0, -1.5, 2.0, 3.0, 4.0, -500000.0,
            ],
            1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, -5.0, 1.0, 2.0, 1.5, 2.0, 3.0, 4.0, -1.0],
            2.0,
        ),
        (
            vec![-1.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, -3.0],
            1.0,
        ),
        (
            vec![1.0, 3.0, 3.0, 3.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (
            vec![4.0, 3.0, 3.0, 4.0, 1.0, 3.0, 2.0, 3.5, 2.0, 3.0, 4.0, 3.0],
            -1.0,
        ),
        (
            vec![2.0, 3.0, 3.0, 4.0, 5.0, 1.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], -2.0),
        (vec![2.0, 3.0, 3.0, -4.0], 2.0),
        (vec![4.0, 3.0, 11.0, -4.0], 3.0),
        (vec![-1.0, 3.0, 2.0, -2.0], 1.0),
        (vec![2.0, -1.0, 2.0, -2.0], 1.0),
        (vec![1.0, -13.0, 2.0, -2.0], 1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            0,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![
                1.0, 3.0, 3.0, 4.0, 5.0, 0.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            0.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 1.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 2.0, 5.0, 3.0, 4.0, -3.5, -2.0, -3.0, -4.0, -5.0,
            ],
            2.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, 2.0, -1.5, -2.0, -3.0, -4.0, -5.0,
            ],
            1.5,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, 3.0, -2.0, -1.5, 2.0, 3.0, 4.0, -500000.0,
            ],
            2.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, 4.0, 5.0, -3.0, 1.0, -1.5, 2.0, 3.0, 4.0, -1.0,
            ],
            3.0,
        ),
        (
            vec![
                2.0, 3.0, 3.0, -4.0, 5.0, 3.0, 1.0, -1.5, 2.0, 3.0, 4.0, -3.0,
            ],
            4.0,
        ),
        (
            vec![-1.0, 3.0, 3.0, -2.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            2.0,
        ),
        (
            vec![-1.0, 3.0, 3.0, -2.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            2.0,
        ),
        (
            vec![-1.0, 3.0, 3.0, 2.0, 5.0, 3.0, 1.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.0,
        ),
        (
            vec![-1.0, 3.0, 3.0, 2.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            -1.5,
        ),
        (
            vec![-1.0, 3.0, 3.0, 0.0, 5.0, 3.0, 2.0, 1.5, 2.0, 3.0, 4.0, 1.0],
            0.0,
        ),
        (vec![2.0, 3.0, 3.0, 4.0], -4.0),
        (vec![2.0, 3.0, 3.0, -4.0], 4.0),
        (vec![2.0, 3.0], f32::NEG_INFINITY),
        (vec![2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, -3.0], f32::NEG_INFINITY),
        (vec![-2.0, 3.0], f32::NEG_INFINITY),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            3,
            8,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
            -5.0,
        ),
        (
            vec![
                -5.0, 22.0, -13.0, 4.0, -5.0, -10.0, 21.0, 11.0, 32.0, 3.0, 14.0, -10.0,
            ],
            5.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            4,
            4,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 2.0, -1.0, 2.0, 3.0, 4.0, 5.0],
        f32::NEG_INFINITY,
    )];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::until_interval(
            4,
            3,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| -a[0])),
        ),
        String::from("until_interval"),
    ));
    run_test(traces_with_expected_value.clone(), signal_name, tbt).unwrap();
}

#[test]
fn test_eventually_globally() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![
                -1.0, -3.0, -3.0, -4.0, -5.0, -1.0, 2.0, 1.5, 2.0, -3.0, -4.0, -5.0,
            ],
            1.5,
        ),
        (
            vec![
                -1.0, -3.0, -3.0, -4.0, -5.0, -1.0, 2.0, 1.5, -1.0, -3.0, -4.0, -5.0,
            ],
            -1.0,
        ),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::leaf(
        Stl::eventually(Stl::globally_interval(
            3,
            5,
            Stl::atomic(vec![signal_name.clone()], Rc::new(|a: &[f32]| a[0])),
        )),
        String::from("eventually_globally"),
    ));
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_sequence() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -0.5, -1.0],
            0.5,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -0.5],
            0.5,
        ),
        (
            vec![0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
            0.5,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, -0.5, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
            0.5,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, -1.5, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0],
            -1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, -1.5, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
            -1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, -1.5, 1.5, -1.0, -1.0, -1.0, -1.0, -1.0],
            -1.5,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            1.0,
        ),
        (vec![1.0], 1.0),
        (vec![-1.0], -1.0),
        (vec![1.0, -1.0], 1.0),
        (vec![1.0, 1.0], 1.0),
        (vec![-1.0, -1.0], -1.0),
        (vec![-1.0, 1.0], -1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::sequence(
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("globally"),
        ),
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| -a[0]),
            )),
            String::from("globally"),
        ),
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_fallback() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0],
            1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            1.0,
        ),
        (
            vec![
                -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            ],
            1.0,
        ),
        (vec![1.0], 1.0),
        (vec![-1.0], 1.0),
        (vec![1.0, -1.0], 1.0),
        (vec![1.0, 1.0], 1.0),
        (vec![-1.0, 1.0], 1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::fallback(vec![
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("globally"),
        ),
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| -a[0]),
            )),
            String::from("globally"),
        ),
    ]));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_parallel() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            0.5,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0],
            1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.1, 1.0, 1.0, 1.0],
            -0.1,
        ),
        (vec![1.0], 0.5),
        (vec![-1.0], 1.0),
        (vec![1.0, -1.0], 1.0),
        (vec![1.0, 1.0], 0.5),
        (vec![-1.0, 1.0], 1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::parallel(
        1,
        vec![
            TbtNode::leaf(
                Stl::globally(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| a[0] - 0.5),
                )),
                String::from("globally"),
            ),
            TbtNode::leaf(
                Stl::eventually(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| -a[0]),
                )),
                String::from("globally"),
            ),
        ],
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            -1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -0.5, 1.0, 1.0, 1.0],
            -1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.6, 1.0, 1.0, 1.0],
            -0.6,
        ),
        (vec![1.0], -1.0),
        (vec![-1.0], -1.5),
        (vec![1.0, -1.0], -1.5),
        (vec![1.0, 1.0], -1.0),
        (vec![-1.0, 1.0], -1.5),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::parallel(
        2,
        vec![
            TbtNode::leaf(
                Stl::globally(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| a[0] - 0.5),
                )),
                String::from("globally"),
            ),
            TbtNode::leaf(
                Stl::eventually(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| -a[0]),
                )),
                String::from("globally"),
            ),
        ],
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name, tbt).unwrap();
}

#[test]
fn test_timeout() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
            1.0,
        ),
        (
            vec![1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            -1.0,
        ),
        (
            vec![-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            -1.0,
        ),
        (vec![1.0], 1.0),
        (vec![-1.0], -1.0),
        (vec![1.0, -1.0], -1.0),
        (vec![1.0, 1.0], 1.0),
        (vec![-1.0, 1.0], -1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::timeout(
        4,
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("globally"),
        ),
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_kleene() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (vec![1.0, 1.0, 1.0, 1.0, 3.0, 4.0], 1.0),
        (vec![1.0, 1.0, 1.0, 1.0, 3.0, 4.0], 1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::kleene(
        2,
        TbtNode::leaf(
            Stl::globally(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("globally"),
        ),
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (vec![-1.0, -1.0, -1.0, 1.0, -3.0, 4.0], 4.0),
        (vec![1.0, -1.0, -1.0, 1.0, -3.0, -4.0], 1.0),
        (vec![-1.0, -1.0, -1.0, 1.0, -3.0, -4.0], 1.0),
        (vec![-1.0], -1.0),
        (vec![1.0, 2.0], 2.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::kleene(
        2,
        TbtNode::leaf(
            Stl::eventually(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("eventually"),
        ),
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![(vec![-1.0, -1.0, -1.0, 1.0, -3.0, 4.0], 4.0)];
    // Define Tree
    let tbt = Tbt::new(TbtNode::kleene_inf(
        TbtNode::leaf(
            Stl::eventually(Stl::atomic(
                vec![signal_name.clone()],
                Rc::new(|a: &[f32]| a[0]),
            )),
            String::from("eventually"),
        ),
        6,
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let traces_with_expected_value = vec![
        (vec![1.0, 1.0, 1.0, 1.0, -3.0, 4.0, 4.0, -4.0], 1.0),
        (vec![-1.0, 1.0, 1.0, 1.0, -3.0, 4.0, 4.0, -4.0], -1.0),
        (vec![1.0, 1.0, 1.0, 1.0, -3.0, 4.0, 4.0, 4.0], 1.0),
        (vec![1.0, 1.0, 1.0, 1.0, 3.0, 4.0, 4.0, 4.0], -1.0),
    ];
    // Define Tree
    let tbt = Tbt::new(TbtNode::kleene_inf(
        TbtNode::sequence(
            TbtNode::leaf(
                Stl::globally(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| a[0]),
                )),
                String::from("globally"),
            ),
            TbtNode::leaf(
                Stl::eventually(Stl::atomic(
                    vec![signal_name.clone()],
                    Rc::new(|a: &[f32]| -a[0]),
                )),
                String::from("eventually"),
            ),
        ),
        6,
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_even() {
    let signal_name = "a".to_string();
    // Defines traces to test
    let trace_length = 10;
    let traces_with_expected_value = vec![
        (vec![0.0, 1.0, 3.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], -1.0),
        (
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            -1.0,
        ),
        (vec![0.0, 1.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], -1.0),
        (vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 1.0),
        (vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0], 1.0),
    ];
    let tbt = Tbt::new(TbtNode::kleene_inf(
        TbtNode::sequence(
            TbtNode::leaf(
                Stl::conjunction(
                    Stl::atomic(
                        vec![signal_name.clone()],
                        Rc::new(|a: &[f32]| {
                            let a = a[0] as i32;
                            if a % 2 == 0 {
                                1.0
                            } else {
                                -1.0
                            }
                        }),
                    ),
                    Stl::neg(Stl::eventually_interval(
                        1,
                        1,
                        Stl::atomic(vec![signal_name.clone()], Rc::new(|_: &[f32]| 1.0)),
                    )),
                ),
                String::from("even"),
            ),
            TbtNode::leaf(
                Stl::neg(Stl::eventually_interval(
                    1,
                    1,
                    Stl::atomic(vec![signal_name.clone()], Rc::new(|_: &[f32]| 1.0)),
                )),
                String::from("true"),
            ),
        ),
        trace_length,
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
    // Defines traces to test
    let trace_length = 9;
    let traces_with_expected_value = vec![
        (vec![0.0, 1.0, 3.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], -1.0),
        (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], -1.0),
        (vec![0.0, 1.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], -1.0),
        (vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], 1.0),
        (vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0], 1.0),
    ];
    let tbt = Tbt::new(TbtNode::kleene_inf(
        TbtNode::sequence(
            TbtNode::leaf(
                Stl::conjunction(
                    Stl::atomic(
                        vec![signal_name.clone()],
                        Rc::new(|a: &[f32]| {
                            let a = a[0] as i32;
                            if a % 2 == 0 {
                                1.0
                            } else {
                                -1.0
                            }
                        }),
                    ),
                    Stl::neg(Stl::eventually_interval(
                        1,
                        1,
                        Stl::atomic(vec![signal_name.clone()], Rc::new(|_: &[f32]| 1.0)),
                    )),
                ),
                String::from("even"),
            ),
            TbtNode::leaf(
                Stl::neg(Stl::eventually_interval(
                    1,
                    1,
                    Stl::atomic(vec![signal_name.clone()], Rc::new(|_: &[f32]| 1.0)),
                )),
                // ),
                String::from("true"),
            ),
        ),
        trace_length,
    ));
    // Run test
    run_test(traces_with_expected_value, signal_name.clone(), tbt).unwrap();
}

#[test]
fn test_handcoded() {
    let specification = "specification/shiplanding_formula_combined.tbt";
    println!("Testing: res/logs_wind_front_Lateral/");
    test_hardcoded_maneuver(specification, "res/logs_wind_front_Lateral/");
    println!("Testing: res/logs_wind_front_45Deg/");
    test_hardcoded_maneuver(specification, "res/logs_wind_front_45Deg/");
    println!("Testing: res/logs_wind_front_Oblique/");
    test_hardcoded_maneuver(specification, "res/logs_wind_front_Oblique/");
    println!("Testing: res/logs_wind_front_Straight/");
    test_hardcoded_maneuver(specification, "res/logs_wind_front_Straight/");
    println!("Testing: res/logs_wind_side_Lateral/");
    test_hardcoded_maneuver(specification, "res/logs_wind_side_Lateral/");
    println!("Testing: res/logs_wind_side_45Deg/");
    test_hardcoded_maneuver(specification, "res/logs_wind_side_45Deg/");
    println!("Testing: res/logs_wind_side_Oblique/");
    test_hardcoded_maneuver(specification, "res/logs_wind_side_Oblique/");
    println!("Testing: res/logs_wind_side_Straight/");
    test_hardcoded_maneuver(specification, "res/logs_wind_side_Straight/");
}
