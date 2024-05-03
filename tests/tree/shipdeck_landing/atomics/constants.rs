// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

pub enum Deg45 {
    HeightAboveShip,
    DistanceToShip,
    AngleToShip,
    AboveTouchdown,
}

impl Deg45 {
    pub fn value(&self) -> f32 {
        match *self {
            Deg45::HeightAboveShip => 20.0,
            Deg45::DistanceToShip => 30.0,
            Deg45::AngleToShip => 135.0,
            Deg45::AboveTouchdown => 20.0,
        }
    }
}

pub enum Lateral {
    HeightAboveShip,
    DistanceToShip,
    AngleToShip,
    AboveTouchdown,
}

impl Lateral {
    pub fn value(&self) -> f32 {
        match *self {
            Lateral::HeightAboveShip => 20.0,
            Lateral::DistanceToShip => 20.0,
            Lateral::AngleToShip => 90.0,
            Lateral::AboveTouchdown => 20.0,
        }
    }
}

#[allow(clippy::enum_variant_names)]
pub enum Oblique {
    HeightAboveShip,
    DistanceToShip,
    AngleToShip,
    AngleOblique,
    AboveTouchdown,
}

impl Oblique {
    pub fn value(&self) -> f32 {
        match *self {
            Oblique::HeightAboveShip => 20.0,
            Oblique::DistanceToShip => 30.0,
            Oblique::AngleToShip => 135.0,
            Oblique::AngleOblique => 45.0,
            Oblique::AboveTouchdown => 20.0,
        }
    }
}

pub enum Straight {
    HeightAboveShip,
    DistanceToShip,
    AngleToShip,
    AboveTouchdown,
}

impl Straight {
    pub fn value(&self) -> f32 {
        match *self {
            Straight::HeightAboveShip => 20.0,
            Straight::DistanceToShip => 20.0,
            Straight::AngleToShip => 180.0,
            Straight::AboveTouchdown => 20.0,
        }
    }
}
