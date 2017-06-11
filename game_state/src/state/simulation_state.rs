// All global state for simulation
pub struct Simulation {
    layers: Vec<SimulationLayer>
}

impl Simulation {
    pub fn new() -> Self{
        Simulation{layers:Vec::new()}
    }

    pub fn layers(&self) -> &Vec<SimulationLayer> {
        &self.layers
    }
}

pub struct SimulationLayer{}
