// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{
    behaviortree::{tbt_node_reset_count, Tbt, TbtNode},
    csv_reader::read_csv_file,
    stl::{stl_reset_count, Stl},
    tree::shipdeck_landing::{
        atomics::descend_touchdown::descend_touchdown, deg45_maneuver::get_45deg_maneuver,
        lateral_maneuver::get_lateral_maneuver, oblique_maneuver::get_oblique_maneuver,
        straight_maneuver::get_straight_maneuver,
    },
    ProvidesTraceAndTree, Trace, UserProvidedFunction,
};
use std::{collections::HashMap, rc::Rc};

impl ProvidesTraceAndTree for UserProvidedFunction {
    fn get_trace(logfile: &str, number_skipped_entries: usize) -> Trace {
        // Ship
        let ship_file = format!("{logfile}SIMOUT_Ship.csv");
        let mut trace_ship_xg = read_csv_file(&ship_file, "xg", number_skipped_entries);
        let mut trace_ship_yg = read_csv_file(&ship_file, "yg", number_skipped_entries);
        let mut trace_ship_zg = read_csv_file(&ship_file, "zg", number_skipped_entries);
        let trace_ship_ug = read_csv_file(&ship_file, "ug", number_skipped_entries);
        let trace_ship_vg = read_csv_file(&ship_file, "vg", number_skipped_entries);
        let trace_ship_wg = read_csv_file(&ship_file, "wg", number_skipped_entries);
        let trace_ship_psi = read_csv_file(&ship_file, "psi", number_skipped_entries);
        for i in 0..trace_ship_xg.len() {
            // Compute position of touchdown point
            trace_ship_zg[i] = trace_ship_zg[i] * -1.0 + 5.0;
            let computed_angle_in_radian = f32::to_radians(180.0) + trace_ship_psi[i];
            trace_ship_xg[i] += 60.0 * f32::cos(computed_angle_in_radian);
            trace_ship_yg[i] += 60.0 * f32::sin(computed_angle_in_radian);
        }
        // UAS
        let uas_file = format!("{logfile}SIMOUT_UAS.csv");
        let trace_uas_xg = read_csv_file(&uas_file, "xg", number_skipped_entries);
        let trace_uas_yg = read_csv_file(&uas_file, "yg", number_skipped_entries);
        let mut trace_uas_zg = read_csv_file(&uas_file, "zg", number_skipped_entries);
        let trace_uas_ug = read_csv_file(&uas_file, "ug", number_skipped_entries);
        let trace_uas_vg = read_csv_file(&uas_file, "vg", number_skipped_entries);
        let trace_uas_wg = read_csv_file(&uas_file, "wg", number_skipped_entries);
        let trace_uas_psi = read_csv_file(&uas_file, "psi", number_skipped_entries);
        for zg in &mut trace_uas_zg {
            *zg = -*zg;
        }
        assert_eq!(trace_ship_xg.len(), trace_uas_xg.len());
        let trace: Trace = (
            trace_ship_xg.len(),
            HashMap::from([
                ("ship_x".to_string(), trace_ship_xg),
                ("ship_y".to_string(), trace_ship_yg),
                ("ship_z".to_string(), trace_ship_zg),
                ("ship_u".to_string(), trace_ship_ug),
                ("ship_v".to_string(), trace_ship_vg),
                ("ship_w".to_string(), trace_ship_wg),
                ("ship_heading".to_string(), trace_ship_psi),
                ("uas_x".to_string(), trace_uas_xg),
                ("uas_y".to_string(), trace_uas_yg),
                ("uas_z".to_string(), trace_uas_zg),
                ("uas_u".to_string(), trace_uas_ug),
                ("uas_v".to_string(), trace_uas_vg),
                ("uas_w".to_string(), trace_uas_wg),
                ("uas_heading".to_string(), trace_uas_psi),
            ]),
        );
        trace
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
}
