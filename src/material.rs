#![allow(dead_code)]

use crate::{my_vec3::MyVec3, scatter::ScatteringType};

//
#[derive(Debug, Copy, Clone, Default)]

pub struct Material {
    pub surface:             ScatteringType,
    pub gain:                MyVec3,
    pub metal_fuzz:          Option<f64>,
    pub index_of_refraction: Option<f64>
}


pub const GLASS:              Material = Material{surface: ScatteringType::RefractiveScattering, gain: MyVec3 {x: 1.0, y: 1.0, z: 1.0}, metal_fuzz:      None, index_of_refraction: Some(1.5)};
pub const PERFECT_REFLECTION: Material = Material{surface: ScatteringType::MetallicScattering,   gain: MyVec3 {x: 1.0, y: 1.0, z: 1.0}, metal_fuzz: Some(0.0), index_of_refraction: None};
pub const YELLOW_TINT:        Material = Material{surface: ScatteringType::DiffuseScattering,    gain: MyVec3 {x: 0.7, y: 0.5, z: 0.2}, metal_fuzz:      None, index_of_refraction: None};
pub const PURE_RED:           Material = Material{surface: ScatteringType::DiffuseScattering,    gain: MyVec3 {x: 1.0, y: 0.0, z: 0.0}, metal_fuzz:      None, index_of_refraction: None};
pub const PURE_GREEN:         Material = Material{surface: ScatteringType::DiffuseScattering,    gain: MyVec3 {x: 0.0, y: 1.0, z: 0.0}, metal_fuzz:      None, index_of_refraction: None};
pub const PURE_BLUE:          Material = Material{surface: ScatteringType::DiffuseScattering,    gain: MyVec3 {x: 0.0, y: 0.0, z: 1.0}, metal_fuzz:      None, index_of_refraction: None};
pub const NEUTRAL_GREY:       Material = Material{surface: ScatteringType::DiffuseScattering,    gain: MyVec3 {x: 0.5, y: 0.5, z: 1.5}, metal_fuzz:      None, index_of_refraction: None};