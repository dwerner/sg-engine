use {
    create_next_identity,
    Identity,
    Identifyable,
    Renderable,
    model,
};

pub struct Thing {
    pub id: Identity,
    pub facets: Vec<Facet>
}

impl Thing {
     pub fn new(facets: Vec<Facet>) -> Self {
         let id = create_next_identity();
         Thing{ id, facets }
     }
}

impl Identifyable for Thing {
    fn identify(&self) -> u64 { self.id }
}

pub struct ThingBuilder {
    facets: Vec<Facet>
}

impl ThingBuilder {
    pub fn build(self) -> Thing {
        Thing::new(self.facets)
    }
    pub fn with_facet(mut self, facet: Facet) -> Self {
        self.facets.push(facet);
        self
    }
}

use std::sync::Arc;

pub enum Facet {
    Input,   //
    Physical(PhysicalFacet<f32>), // does it have mass?
    Network,
    Health(HealthFacet),  // can it be hurt? die?
    Pathing, // finding it's way around
    Camera,
    Dialogue, // can this entity be talked with?
    Mesh(Arc<model::Model>),
    AI,
    UI,
}

use cgmath::Vector3;

pub enum Shape<U> {
    Box { width: U, height: U, depth: U },
    Cone { radius: U, height: U },
    Cylinder { radius: U, height: U },
    Sphere { radius: U },
}

pub struct HealthFacet {
    hp: u32
}

impl HealthFacet {
    fn take_dmg(&mut self, dmg: u32) {
        if dmg > self.hp {
            self.hp = 0;
        } else {
            self.hp -= dmg;
        }
    }
    fn is_alive(&self) -> bool { self.hp > 0 }
}

pub struct PhysicalFacet<U> {
    pub body: Shape<U>,
    pub mass: U,
    pub linear_velocity: Vector3<U>,
    pub angular_velocity: Vector3<U>, // is this sufficient for angular velocity? durrrrr
    pub position: Vector3<U>,
}




