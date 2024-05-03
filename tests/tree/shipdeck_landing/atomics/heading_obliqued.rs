// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

const SLACK: f32 = 1.0;

pub fn heading_obliqued(oblique_angle: f32, heading_information: &[f32]) -> f32 {
    let uas_heading = heading_information[0]; // Psi
    let ship_heading = heading_information[1]; // Psi
    SLACK - f32::abs(f32::abs(f32::to_degrees(uas_heading - ship_heading)) - oblique_angle)
}
