// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

const SLACK: f32 = 2.5;

pub fn in_position(
    height_above_ship: f32,
    distance_to_ship: f32,
    angle_to_ship: f32,
    position_information: &[f32],
) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let ship_x = position_information[3];
    let ship_y = position_information[4];
    let ship_z = position_information[5];
    let ship_heading = position_information[6]; //* 180.0 / std::f32::consts::PI;

    // Compute ideal position
    let (best_x, best_y, best_z) = get_best_position(
        height_above_ship,
        distance_to_ship,
        angle_to_ship,
        ship_x,
        ship_y,
        ship_z,
        ship_heading,
    );
    SLACK
        - f32::sqrt(
            (best_x - uas_x).powf(2.0) + (best_y - uas_y).powf(2.0) + (best_z - uas_z).powf(2.0),
        )
}

pub fn get_best_position(
    height_above_ship: f32,
    distance_to_ship: f32,
    angle_to_ship: f32,
    ship_x: f32,
    ship_y: f32,
    ship_z: f32,
    ship_heading: f32,
) -> (f32, f32, f32) {
    let computed_angle_in_radian = f32::to_radians(angle_to_ship) + ship_heading;
    let x_best = ship_x + distance_to_ship * f32::cos(computed_angle_in_radian);
    let y_best = ship_y + distance_to_ship * f32::sin(computed_angle_in_radian);
    let z_best = ship_z + height_above_ship;
    (x_best, y_best, z_best)
}
