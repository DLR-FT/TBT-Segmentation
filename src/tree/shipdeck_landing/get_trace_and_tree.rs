// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{
    behaviortree::{tbt_node_reset_count, Tbt, TbtNode},
    csv_reader::read_csv_file,
    stl::{stl_reset_count, Stl},
    tree::shipdeck_landing::{
        atomics::{
            constants,
            descend_touchdown::descend_touchdown,
            in_position::{self},
        },
        deg45_maneuver::get_45deg_maneuver,
        lateral_maneuver::get_lateral_maneuver,
        oblique_maneuver::get_oblique_maneuver,
        straight_maneuver::get_straight_maneuver,
    },
    ProvidesTraceAndTree, Trace, UserProvidedFunction,
};
use std::{collections::HashMap, rc::Rc};

impl ProvidesTraceAndTree for UserProvidedFunction {
    fn get_trace(logfile: &str, number_skipped_entries: usize) -> Trace {
        println!(
            "Skipped {number_skipped_entries} of logfile:\n\t{}",
            logfile
        );
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
        println!("Events per second: {}", events_per_second);

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

    fn extract_trace(
        logfile_folder: &str,
        number_skipped_entries: usize,
        maneuver_n: u32,
        lower: usize,
        upper: usize,
    ) -> String {
        println!(
            "Skipped {number_skipped_entries} of logfile:\n\t{}",
            logfile_folder
        );
        // Ship
        let ship_file = format!("{logfile_folder}SIMOUT_Ship.csv");
        let trace_ship_xg = read_csv_file(&ship_file, "xg", number_skipped_entries);
        let trace_ship_yg = read_csv_file(&ship_file, "yg", number_skipped_entries);
        let trace_ship_zg = read_csv_file(&ship_file, "zg", number_skipped_entries);
        let trace_ship_ug = read_csv_file(&ship_file, "ug", number_skipped_entries);
        let trace_ship_vg = read_csv_file(&ship_file, "vg", number_skipped_entries);
        let trace_ship_wg = read_csv_file(&ship_file, "wg", number_skipped_entries);
        let trace_ship_psi = read_csv_file(&ship_file, "psi", number_skipped_entries);
        let mut touchdown_x = Vec::new();
        let mut touchdown_y = Vec::new();
        let mut touchdown_z = Vec::new();
        for i in 0..trace_ship_xg.len() {
            touchdown_z.push(trace_ship_zg[i] * -1.0 + 5.0);
            let computed_angle_in_radian = f32::to_radians(180.0) + trace_ship_psi[i];
            touchdown_x.push(trace_ship_xg[i] + 60.0 * f32::cos(computed_angle_in_radian));
            touchdown_y.push(trace_ship_yg[i] + 60.0 * f32::sin(computed_angle_in_radian));
        }

        // UAS
        let uas_file = format!("{logfile_folder}SIMOUT_UAS.csv");
        let trace_uas_time = read_csv_file(&uas_file, "time", number_skipped_entries);
        let trace_uas_phi = read_csv_file(&uas_file, "phi", number_skipped_entries);
        let trace_uas_theta = read_csv_file(&uas_file, "theta", number_skipped_entries);
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
        let mut trace_best_x = Vec::new();
        let mut trace_best_y = Vec::new();
        let mut trace_best_z = Vec::new();
        let mut change_uas_pos = f32::MIN;
        let mut change_uas_vel = f32::MIN;
        let mut change_uas_h = f32::MIN;
        for i in 0..trace_uas_xg.len() {
            let ship_x = trace_ship_xg[i];
            let ship_y = trace_ship_yg[i];
            let ship_z = trace_ship_zg[i];
            let ship_heading = trace_ship_psi[i];

            let (best_x, best_y, best_z) = match maneuver_n {
                0 => in_position::get_best_position(
                    constants::Deg45::HeightAboveShip.value(),
                    constants::Deg45::DistanceToShip.value(),
                    constants::Deg45::AngleToShip.value(),
                    ship_x,
                    ship_y,
                    ship_z,
                    ship_heading,
                ),
                1 => in_position::get_best_position(
                    constants::Lateral::HeightAboveShip.value(),
                    constants::Lateral::DistanceToShip.value(),
                    constants::Lateral::AngleToShip.value(),
                    ship_x,
                    ship_y,
                    ship_z,
                    ship_heading,
                ),
                2 => in_position::get_best_position(
                    constants::Oblique::HeightAboveShip.value(),
                    constants::Oblique::DistanceToShip.value(),
                    constants::Oblique::AngleToShip.value(),
                    ship_x,
                    ship_y,
                    ship_z,
                    ship_heading,
                ),
                3 => in_position::get_best_position(
                    constants::Straight::HeightAboveShip.value(),
                    constants::Straight::DistanceToShip.value(),
                    constants::Straight::AngleToShip.value(),
                    ship_x,
                    ship_y,
                    ship_z,
                    ship_heading,
                ),
                _ => panic!("Invalid maneuver!"),
            };
            trace_best_x.push(best_x);
            trace_best_y.push(best_y);
            trace_best_z.push(best_z);
            // Compute maximal and minimal changes
            if i > 0 {
                let o_x = trace_uas_xg[i];
                let n_x = trace_uas_xg[i - 1];
                let o_y = trace_uas_yg[i];
                let n_y = trace_uas_yg[i - 1];
                let o_z = trace_uas_zg[i];
                let n_z = trace_uas_zg[i - 1];
                let o_u = trace_uas_ug[i];
                let n_u = trace_uas_ug[i - 1];
                let o_v = trace_uas_vg[i];
                let n_v = trace_uas_vg[i - 1];
                let o_w = trace_uas_wg[i];
                let n_w = trace_uas_wg[i - 1];
                let o_h = trace_uas_psi[i];
                let n_h = trace_uas_psi[i - 1];
                let change_pos =
                    (o_x - n_x).powf(2.0) + (o_y - n_y).powf(2.0) + (o_z - n_z).powf(2.0);
                let change_vel =
                    (o_u - n_u).powf(2.0) + (o_v - n_v).powf(2.0) + (o_w - n_w).powf(2.0);
                let change_h = f32::abs(o_h - n_h);
                change_uas_pos = f32::max(change_uas_pos, change_pos);
                change_uas_vel = f32::max(change_uas_vel, change_vel);
                change_uas_h = f32::max(change_uas_h, change_h);
            }
        }

        assert_eq!(trace_ship_xg.len(), trace_best_x.len());
        let header = "time,phi,theta,uas_x,uas_y,uas_z,uas_u,uas_v,uas_w,uas_h,ship_x,ship_y,ship_z,td_x,td_y,td_z,ship_u,ship_v,ship_w,ship_h,best_x,best_y,best_z,c_pos,c_vel,c_h\n".to_string();
        let mut rows = String::new();
        for i in lower..(upper + 1) {
            rows += &format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                trace_uas_time[i],
                trace_uas_phi[i],
                trace_uas_theta[i],
                trace_uas_xg[i],
                trace_uas_yg[i],
                trace_uas_zg[i],
                trace_uas_ug[i],
                trace_uas_vg[i],
                trace_uas_wg[i],
                trace_uas_psi[i],
                trace_ship_xg[i],
                trace_ship_yg[i],
                trace_ship_zg[i],
                touchdown_x[i],
                touchdown_y[i],
                touchdown_z[i],
                trace_ship_ug[i],
                trace_ship_vg[i],
                trace_ship_wg[i],
                trace_ship_psi[i],
                trace_best_x[i],
                trace_best_y[i],
                trace_best_z[i],
                change_uas_pos,
                change_uas_vel,
                change_uas_h,
            );
        }
        header + &rows
    }
}
