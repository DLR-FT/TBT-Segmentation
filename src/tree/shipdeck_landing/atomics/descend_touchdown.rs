// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

const SLACK: f32 = 1.0;

pub fn descend_touchdown(position_information: &[f32]) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let touchdown_x = position_information[3];
    let touchdown_y = position_information[4];
    let touchdown_z = position_information[5];
    SLACK
        - f32::sqrt(
            (touchdown_x - uas_x).powf(2.0)
                + (touchdown_y - uas_y).powf(2.0)
                + (touchdown_z - uas_z).powf(2.0),
        )
}
