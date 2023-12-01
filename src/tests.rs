// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{
    behaviortree::{Tbt, TbtNode},
    evaluate,
    stl::Stl,
    Trace,
};
use std::{collections::HashMap, rc::Rc, time::SystemTime};

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
        if robustness == expected {
            continue;
        } else {
            return Err(format!("Expected {expected} but was {robustness}."));
        }
    }
    Ok(())
}

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
