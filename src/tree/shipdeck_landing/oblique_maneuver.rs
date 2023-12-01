// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use super::atomics::combined::{combined_inpos_ho_va, combined_moveto_ho};
use super::atomics::constants::Oblique;
use super::atomics::in_position::in_position;
use crate::{behaviortree::TbtNode, stl::Stl};
use std::rc::Rc;

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn get_oblique_maneuver(
    events_per_second: &u64,
    uas_x: &str,
    uas_y: &str,
    uas_z: &str,
    uas_u: &str,
    uas_v: &str,
    uas_w: &str,
    uas_heading: &str,
    ship_x: &str,
    ship_y: &str,
    ship_z: &str,
    ship_u: &str,
    ship_v: &str,
    ship_w: &str,
    ship_heading: &str,
) -> TbtNode {
    let move_to_position = TbtNode::leaf(
        Stl::eventually(Stl::atomic(
            vec![
                uas_x.to_owned(),
                uas_y.to_owned(),
                uas_z.to_owned(),
                ship_x.to_owned(),
                ship_y.to_owned(),
                ship_z.to_owned(),
                ship_heading.to_owned(),
            ],
            Rc::new(|arguments| {
                in_position(
                    Oblique::HeightAboveShip.value(),
                    Oblique::DistanceToShip.value(),
                    Oblique::AngleToShip.value(),
                    arguments,
                )
            }),
        )),
        String::from("move_to_position_oblique"),
    );

    let stay_in_position = TbtNode::leaf(
        Stl::globally_interval(
            0,
            (events_per_second * 5).try_into().unwrap(), // for five seconds
            Stl::atomic(
                vec![
                    uas_x.to_owned(),
                    uas_y.to_owned(),
                    uas_z.to_owned(),
                    uas_u.to_owned(),
                    uas_v.to_owned(),
                    uas_w.to_owned(),
                    uas_heading.to_owned(),
                    ship_x.to_owned(),
                    ship_y.to_owned(),
                    ship_z.to_owned(),
                    ship_u.to_owned(),
                    ship_v.to_owned(),
                    ship_w.to_owned(),
                    ship_heading.to_owned(),
                ],
                Rc::new(|arguments| {
                    combined_inpos_ho_va(
                        Oblique::HeightAboveShip.value(),
                        Oblique::DistanceToShip.value(),
                        Oblique::AngleToShip.value(),
                        Oblique::AngleOblique.value(),
                        arguments,
                    )
                }),
            ),
        ),
        String::from("stay_in_position"),
    );

    let move_to_touchdown_oblique = TbtNode::leaf(
        Stl::eventually(Stl::atomic(
            vec![
                uas_x.to_owned(),
                uas_y.to_owned(),
                uas_z.to_owned(),
                uas_u.to_owned(),
                uas_v.to_owned(),
                uas_w.to_owned(),
                uas_heading.to_owned(),
                ship_x.to_owned(),
                ship_y.to_owned(),
                ship_z.to_owned(),
                ship_u.to_owned(),
                ship_v.to_owned(),
                ship_w.to_owned(),
                ship_heading.to_owned(),
            ],
            Rc::new(|arguments| {
                combined_moveto_ho(
                    Oblique::AboveTouchdown.value(),
                    Oblique::AngleOblique.value(),
                    arguments,
                )
            }),
        )),
        String::from("move_to_touchdown"),
    );

    TbtNode::sequence(
        move_to_position,
        TbtNode::sequence(stay_in_position, move_to_touchdown_oblique),
    )
}
