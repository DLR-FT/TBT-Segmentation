// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use super::atomics::combined::combined_movetp_ha;
use super::atomics::in_position::in_position;
use super::atomics::{combined::combined_inpos_ha_va, constants::Deg45};
use crate::{behaviortree::TbtNode, stl::Stl};
use std::rc::Rc;

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn get_45deg_maneuver(
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
                    Deg45::HeightAboveShip.value(),
                    Deg45::DistanceToShip.value(),
                    Deg45::AngleToShip.value(),
                    arguments,
                )
            }),
        )),
        String::from("move_to_position_45deg"),
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
                    combined_inpos_ha_va(
                        Deg45::HeightAboveShip.value(),
                        Deg45::DistanceToShip.value(),
                        Deg45::AngleToShip.value(),
                        arguments,
                    )
                }),
            ),
        ),
        String::from("stay_in_position"),
    );

    let move_to_touchdown = TbtNode::leaf(
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
            Rc::new(|arguments| combined_movetp_ha(Deg45::AboveTouchdown.value(), arguments)),
        )),
        String::from("move_to_touchdown"),
    );

    TbtNode::sequence(
        move_to_position,
        TbtNode::sequence(stay_in_position, move_to_touchdown),
    )
}
