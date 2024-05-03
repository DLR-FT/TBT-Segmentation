// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

const SLACK: f32 = 2.5;

pub fn move_to_touchdown(height_above_ship: f32, position_information: &[f32]) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let above_touchdown_x = position_information[3];
    let above_touchdown_y = position_information[4];
    let above_touchdown_z = position_information[5] + height_above_ship;
    SLACK
        - f32::sqrt(
            (above_touchdown_x - uas_x).powf(2.0)
                + (above_touchdown_y - uas_y).powf(2.0)
                + (above_touchdown_z - uas_z).powf(2.0),
        )
}
