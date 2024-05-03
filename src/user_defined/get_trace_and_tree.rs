// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{csv_reader::read_csv_file, ProvidesTraceAndTree, Trace, UserProvidedFunction};
use std::collections::HashMap;

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

    fn get_events_per_second(number_skipped_entries: usize) -> u64 {
        let frequency = if number_skipped_entries != 0 {
            0.005 * number_skipped_entries as f32
        } else {
            0.005
        };
        let events_per_second = (1.0 / frequency) as u64;
        events_per_second
    }
}
