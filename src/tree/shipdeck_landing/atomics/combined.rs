// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use super::{
    heading_aligned::heading_aligned, heading_obliqued::heading_obliqued, in_position::in_position,
    move_to_touchdown::move_to_touchdown, velocity_aligned::velocity_aligned,
};

pub fn combined_inpos_ha_va(
    height_above_ship: f32,
    distance_to_ship: f32,
    angle_to_ship: f32,
    position_information: &[f32],
) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let uas_u = position_information[3];
    let uas_v = position_information[4];
    let uas_w = position_information[5];
    let uas_heading = position_information[6]; // Psi
    let ship_x = position_information[7];
    let ship_y = position_information[8];
    let ship_z = position_information[9];
    let ship_u = position_information[10];
    let ship_v = position_information[11];
    let ship_w = position_information[12];
    let ship_heading = position_information[13]; // Psi

    let in_position_robust = in_position(
        height_above_ship,
        distance_to_ship,
        angle_to_ship,
        &[uas_x, uas_y, uas_z, ship_x, ship_y, ship_z, ship_heading],
    );

    let heading_aligned_robust = heading_aligned(&[uas_heading, ship_heading]);
    let velocity_aligned_robust = velocity_aligned(&[uas_u, uas_v, uas_w, ship_u, ship_v, ship_w]);

    f32::min(
        in_position_robust,
        f32::min(heading_aligned_robust, velocity_aligned_robust),
    )
}

pub fn combined_movetp_ha(height_above_ship: f32, position_information: &[f32]) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let _uas_u = position_information[3];
    let _uas_v = position_information[4];
    let _uas_w = position_information[5];
    let uas_heading = position_information[6]; // Psi
    let ship_x = position_information[7];
    let ship_y = position_information[8];
    let ship_z = position_information[9];
    let _ship_u = position_information[10];
    let _ship_v = position_information[11];
    let _ship_w = position_information[12];
    let ship_heading = position_information[13]; // Psi

    let move_td_robust = move_to_touchdown(
        height_above_ship,
        &[uas_x, uas_y, uas_z, ship_x, ship_y, ship_z],
    );

    let heading_aligned_robust = heading_aligned(&[uas_heading, ship_heading]);

    f32::min(move_td_robust, heading_aligned_robust)
}

pub fn combined_inpos_ho_va(
    height_above_ship: f32,
    distance_to_ship: f32,
    angle_to_ship: f32,
    oblique_angle: f32,
    position_information: &[f32],
) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let uas_u = position_information[3];
    let uas_v = position_information[4];
    let uas_w = position_information[5];
    let uas_heading = position_information[6]; // Psi
    let ship_x = position_information[7];
    let ship_y = position_information[8];
    let ship_z = position_information[9];
    let ship_u = position_information[10];
    let ship_v = position_information[11];
    let ship_w = position_information[12];
    let ship_heading = position_information[13]; // Psi

    let in_position_robust = in_position(
        height_above_ship,
        distance_to_ship,
        angle_to_ship,
        &[uas_x, uas_y, uas_z, ship_x, ship_y, ship_z, ship_heading],
    );

    let heading_obliqued_robust = heading_obliqued(oblique_angle, &[uas_heading, ship_heading]);
    let velocity_aligned_robust = velocity_aligned(&[uas_u, uas_v, uas_w, ship_u, ship_v, ship_w]);

    f32::min(
        in_position_robust,
        f32::min(heading_obliqued_robust, velocity_aligned_robust),
    )
}

pub fn combined_moveto_ho(
    height_above_ship: f32,
    oblique_angle: f32,
    position_information: &[f32],
) -> f32 {
    let uas_x = position_information[0];
    let uas_y = position_information[1];
    let uas_z = position_information[2];
    let _uas_u = position_information[3];
    let _uas_v = position_information[4];
    let _uas_w = position_information[5];
    let uas_heading = position_information[6]; // Psi
    let ship_x = position_information[7];
    let ship_y = position_information[8];
    let ship_z = position_information[9];
    let _ship_u = position_information[10];
    let _ship_v = position_information[11];
    let _ship_w = position_information[12];
    let ship_heading = position_information[13]; // Psi

    let move_td_robust = move_to_touchdown(
        height_above_ship,
        &[uas_x, uas_y, uas_z, ship_x, ship_y, ship_z],
    );

    let heading_aligned_robust = heading_obliqued(oblique_angle, &[uas_heading, ship_heading]);

    f32::min(move_td_robust, heading_aligned_robust)
}
