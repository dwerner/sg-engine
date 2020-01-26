use nphysics3d::math::Vector;
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct SimulationState {
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub mechanical_world: DefaultMechanicalWorld<f32>,
}

impl Default for SimulationState {
    fn default() -> Self {
        SimulationState {
            geometrical_world: DefaultGeometricalWorld::new(),
            mechanical_world: DefaultMechanicalWorld::new(Vector::new(0.0, 1.0, 0.0)),
        }
    }
}
