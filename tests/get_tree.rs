use std::rc::Rc;

use tbt_segmentation::behaviortree::{tbt_node_reset_count, Tbt, TbtNode};
use tbt_segmentation::stl::{stl_reset_count, Stl};
pub mod tree {
    pub mod shipdeck_landing {
        pub mod deg45_maneuver;
        pub mod lateral_maneuver;
        pub mod oblique_maneuver;
        pub mod straight_maneuver;
        pub mod atomics {
            pub mod combined;
            pub mod constants;
            pub mod descend_touchdown;
            pub mod heading_aligned;
            pub mod heading_obliqued;
            pub mod in_position;
            pub mod move_to_touchdown;
            pub mod velocity_aligned;
        }
    }
}

use tree::shipdeck_landing::atomics::descend_touchdown::descend_touchdown;
use tree::shipdeck_landing::deg45_maneuver::get_45deg_maneuver;
use tree::shipdeck_landing::lateral_maneuver::get_lateral_maneuver;
use tree::shipdeck_landing::oblique_maneuver::get_oblique_maneuver;
use tree::shipdeck_landing::straight_maneuver::get_straight_maneuver;

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
