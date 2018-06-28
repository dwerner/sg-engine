use {
    create_next_identity,
    Identity,
    Identifyable,
    model,
};

use cgmath::Vector3;
use cgmath::Matrix4;

use std::sync::{
    Arc,
    Mutex,
};


pub enum FacetIndex {
    Physical(usize), // does it have mass?
    Health(usize),  // can it be hurt? die?
    Camera(usize),
    Model(usize),
    // Input(usize),
    // Network(usize),
    // Pathing(usize), // finding it's way around
    // Dialogue(usize), // can this entity be talked with?
    // AI(usize),
}

pub struct ModelInstanceFacet<U = f32> {
    pub transform: Matrix4<U>,
    model: Arc<model::Model>
}

pub struct HealthFacet {
    hp: u32
}

impl HealthFacet {
    fn new(hp: u32) -> Self {
        HealthFacet{ hp }
    }
    fn take_dmg(&mut self, dmg: u32) {
        if dmg > self.hp {
            self.hp = 0;
        } else {
            self.hp -= dmg;
        }
    }
    fn is_alive(&self) -> bool { self.hp > 0 }
}

pub enum Shape<U> {
    Box { width: U, height: U, depth: U },
    Cone { radius: U, height: U },
    Cylinder { radius: U, height: U },
    Sphere { radius: U },
}

pub struct PhysicalFacet<U> {
    pub body: Shape<U>,
    pub mass: U,
    pub linear_velocity: Vector3<U>,
    pub angular_velocity: Vector3<U>, // is this sufficient for angular velocity? durrrrr
    pub position: Vector3<U>,
}

pub struct CameraFacet<U> {
    pub view: Matrix4<U>,
    pub transform: Matrix4<U>
}

impl <U> CameraFacet<U> {
    pub fn new(view: Matrix4<U>, transform: Matrix4<U>) -> Self {
        CameraFacet{view, transform}
    }
}

// TODO implement the rest of the facets
// the main idea here is to construct contiguous areas in memory for different facets
// this is a premature optimization for the Thing/Facet system in general to avoid losing cache
// coherency whilst traversing a series of objects. Probably we want to integrate concurrency
// safety here.
pub struct WorldFacets {
    pub cameras: Vec<CameraFacet<f32>>,
    pub models: Vec<ModelInstanceFacet>,
    pub physical: Vec<PhysicalFacet<f32>>,
    pub health: Vec<HealthFacet>,
}

impl WorldFacets {
    pub fn new() -> Self {
        WorldFacets{
            cameras: Vec::new(),
            physical: Vec::new(),
            models: Vec::new(),
            health: Vec::new(),
        }
    }
}

pub struct World {
    things: Vec<Arc<Mutex<Thing>>>,
    facets: WorldFacets,
}

impl World {

    pub fn new() -> Self {
        World{
            things: Vec::new(),
            facets: WorldFacets::new()
        }
    }

    pub fn start_thing(&mut self) -> ThingBuilder {
        ThingBuilder{
            world: self,
            facets: Vec::new()
        }
    }

    pub fn get_things(&self) -> &Vec<Arc<Mutex<Thing>>> {
        &self.things
    }

    pub fn get_facets(&mut self) -> &mut WorldFacets {
        &mut self.facets
    }
}

pub struct ThingBuilder<'a> {
    world: &'a mut World,
    facets: Vec<FacetIndex>,
}

impl <'a> ThingBuilder <'a> {

    pub fn with_camera(mut self, camera: CameraFacet<f32>) -> Self {
        let idx = self.world.facets.cameras.len();
        self.world.facets.cameras.push(camera);
        self.facets.push(FacetIndex::Camera(idx));
        self
    }

    pub fn with_model(mut self, transform: Matrix4<f32>, model: Arc<model::Model>) -> Self {
        let idx = self.world.facets.models.len();
        self.world.facets.models.push(ModelInstanceFacet{ transform, model });
        self.facets.push(FacetIndex::Camera(idx));
        self
    }

    pub fn build(self) -> Arc<Mutex<Thing>> {
        let thing = Thing::new(self.facets);
        let a = Arc::new(Mutex::new(thing));
        self.world.things.push(a.clone());
        a
    }

}

pub struct Thing {
    pub id: Identity,
    pub facets: Vec<FacetIndex>, // index pointers to WorldFacets' specific fields
}

impl Thing {

    pub fn new(facets: Vec<FacetIndex>) -> Self {
        let id = create_next_identity();
        Thing{ id, facets }
    }

    pub fn get_camera_fi(&mut self) -> Option<&FacetIndex> {
        self.facets.iter().find(|i| {
            if let FacetIndex::Camera(_) = i { true } else { false }
        })
    }

}

impl Identifyable for Thing {
    fn identify(&self) -> u64 { self.id }
}

