// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use std::fs::File;
use std::io::Write;

use crate::behaviortree::ClonedSegmentation;

#[derive(Serialize)]
struct Segment {
    lower: usize,
    upper: usize,
    rho: f32,
    details: SegmentDetail,
}

#[derive(Serialize)]
struct SegmentDetail {
    id: usize,
    description: String,
}

#[derive(Serialize)]
struct NamedSegmentation(String, TomlSegmentation);

#[derive(Serialize)]
struct TomlSegmentation {
    delta: usize,
    robustness: f32,
    segments: Vec<Segment>,
}

#[derive(Serialize)]
struct Segmentations {
    segmentations: Vec<NamedSegmentation>,
}

pub fn generate_toml(
    location: String,
    delta: usize,
    best_segmentation: ClonedSegmentation,
    alternative_segmentation: Option<Vec<ClonedSegmentation>>,
) -> std::io::Result<()> {
    let mut segmentations = Vec::<NamedSegmentation>::new();
    // Adding best segmentation
    segmentations.push(read_segmentation(
        best_segmentation,
        delta,
        "best".to_string(),
    ));
    // Adding the alternative segmentations
    if let Some(alternatives) = alternative_segmentation {
        alternatives.into_iter().enumerate().for_each(|(i, seg)| {
            segmentations.push(read_segmentation(
                seg,
                delta,
                format!("alternative_{}", i + 1),
            ))
        })
    }
    let all_segmentations = Segmentations { segmentations };
    let toml_string =
        toml::to_string_pretty(&all_segmentations).expect("Failed to serialize to TOML");
    let mut file = File::create(location)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

fn read_segmentation(
    segmentation: (f32, Vec<(crate::behaviortree::TbtNode, usize, usize, f32)>),
    delta: usize,
    name: String,
) -> NamedSegmentation {
    let mut segments = Vec::<Segment>::new();
    for (node, lower, upper, rho) in segmentation.1 {
        let details = SegmentDetail {
            id: node.get_index(),
            description: node.pretty_print(false, 0),
        };
        let segment = Segment {
            lower,
            upper,
            rho,
            details,
        };
        segments.push(segment);
    }
    NamedSegmentation(
        name,
        TomlSegmentation {
            delta,
            robustness: segmentation.0,
            segments,
        },
    )
}
