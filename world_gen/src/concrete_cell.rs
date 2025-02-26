use std::fmt::Display;

use crate::biome::BiomeType;

#[derive(Debug, Clone)]
pub enum BuildingElement {
    BuildingTopLeft,
    BuildingTopRight,
    BuildingBottomLeft,
    BuildingBottomRight,
    BuildingHorizontal,
    BuildingVertical,
    BuildingInterior,
    Road,
}

#[derive(Debug, Clone)]
pub struct BuildingPart {
    pub element: BuildingElement,
    pub is_door: bool,
}

impl Display for BuildingPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.element {
            BuildingElement::BuildingTopLeft if self.is_door => write!(f, "╔"),
            BuildingElement::BuildingTopRight if self.is_door => write!(f, "╗"),
            BuildingElement::BuildingBottomLeft if self.is_door => write!(f, "╚"),
            BuildingElement::BuildingBottomRight if self.is_door => write!(f, "╝"),
            BuildingElement::BuildingHorizontal if self.is_door => write!(f, "═"),
            BuildingElement::BuildingVertical if self.is_door => write!(f, "║"),

            BuildingElement::BuildingTopLeft => write!(f, "┌"),
            BuildingElement::BuildingTopRight => write!(f, "┐"),
            BuildingElement::BuildingBottomLeft => write!(f, "└"),
            BuildingElement::BuildingBottomRight => write!(f, "┘"),
            BuildingElement::BuildingHorizontal => write!(f, "─"),
            BuildingElement::BuildingVertical => write!(f, "│"),
            BuildingElement::BuildingInterior => write!(f, "█"),
            BuildingElement::Road => write!(f, "░"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConcreteCell {
    pub biome: BiomeType,
    pub building_part: Option<BuildingPart>,
}
