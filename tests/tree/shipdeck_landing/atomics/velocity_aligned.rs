// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

const SLACK: f32 = 2.0;

pub fn velocity_aligned(velocity_information: &[f32]) -> f32 {
    let uas_velocity = f32::sqrt(
        (velocity_information[0]).powf(2.0)
            + (velocity_information[1]).powf(2.0)
            + (velocity_information[2]).powf(2.0),
    );
    let ship_velocity = f32::sqrt(
        (velocity_information[3]).powf(2.0)
            + (velocity_information[4]).powf(2.0)
            + (velocity_information[5]).powf(2.0),
    );
    SLACK - f32::abs(uas_velocity - ship_velocity)
}
