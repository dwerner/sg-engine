use std::sync::{Arc, Mutex};
use std::time::Duration;

use nalgebra::{Matrix4, Perspective3, Scalar, Vector3};

use crate::{create_next_identity, model, Identifyable, Identity};

#[derive(Copy, Clone)]
pub enum FacetIndex {
    Physical(usize), // does it have mass?
    Health(usize),   // can it be hurt? die?
    Camera(usize),
    Model(usize),
    // Input(usize),
    // Network(usize),
    // Pathing(usize), // finding it's way around
    // Dialogue(usize), // can this entity be talked with?
    // AI(usize),
}

pub struct ModelInstanceFacet<U = f32>
where
    U: Scalar,
{
    pub transform: Matrix4<U>,
    pub model: Arc<model::Model>,
}

pub struct HealthFacet {
    pub hp: u32,
}

impl HealthFacet {
    pub fn new(hp: u32) -> Self {
        HealthFacet { hp }
    }
    pub fn take_dmg(&mut self, dmg: u32) {
        if dmg > self.hp {
            self.hp = 0;
        } else {
            self.hp -= dmg;
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

pub enum Shape {
    Box { width: f32, height: f32, depth: f32 },
    Cone { radius: f32, height: f32 },
    Cylinder { radius: f32, height: f32 },
    Sphere { radius: f32 },
}

pub struct PhysicalFacet {
    pub body: Shape,
    pub mass: f32,
    pub linear_velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>, // is this sufficient for angular velocity? durrrrr
    pub position: Vector3<f32>,
}

pub struct CameraFacet {
    // TODO: pos and rotation should be part of PhysicalFacet
    pub pos: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,

    _dirty: bool,
    pub rotation_speed: f32,
    pub movement_speed: f32,

    pub view: Matrix4<f32>,
    pub perspective: Perspective3<f32>,
    pub movement_dir: Option<Direction>,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}

impl CameraFacet {
    pub fn new(pos: Vector3<f32>, pitch: f32, yaw: f32) -> Self {
        let mut c = CameraFacet {
            pos,
            pitch,
            yaw,
            rotation_speed: 1.0,
            movement_speed: 1.0,
            movement_dir: None,
            _dirty: false,
            view: Matrix4::<f32>::identity(),

            // TODO fix default perspective values
            perspective: Perspective3::<f32>::new(
                1.7,    //aspect
                0.75,   //fovy
                0.0,    // near
                1000.0, //far
            ),
        };
        c.update_view_matrix();
        c
    }

    pub fn set_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        self.perspective = Perspective3::<f32>::new(aspect, fov, near, far);
    }

    pub fn update_aspect_ratio(&mut self, aspect: f32) {
        self.perspective.set_aspect(aspect);
    }

    pub fn forward(&self) -> Vector3<f32> {
        let rx = self.pitch;
        let ry = self.yaw;
        Vector3::new(-(rx.cos()) * ry.sin(), rx.sin(), rx.cos() * ry.cos()).normalize()
    }

    pub fn right(&self) -> Vector3<f32> {
        let y = Vector3::new(1.0, 0.0, 0.0);
        let forward = self.forward();
        let cross = y.cross(&forward);
        cross.normalize()
    }

    pub fn up(&self) -> Vector3<f32> {
        let x = Vector3::new(0.0, 1.0, 0.0);
        x.cross(&self.forward()).normalize()
    }

    pub fn update(&mut self, dt: &Duration) {
        let amount = (dt.as_millis() as f64 / 100.0) as f32;
        if let Some(move_dir) = &self.movement_dir {
            let m = self.movement_speed * amount;
            let d = match move_dir {
                Direction::Forward => self.forward(),
                Direction::Backward => -self.forward(),
                Direction::Right => self.right(),
                Direction::Left => -self.right(),
                Direction::Up => self.up(),
                Direction::Down => -self.up(),
            };
            self.pos += d * m;
        }
        self.update_view_matrix();
    }

    pub fn update_view_matrix(&mut self) {
        let rot = Matrix4::from_euler_angles(self.pitch, self.yaw, 0.0);
        let trans = Matrix4::new_translation(&self.pos);
        self.view = trans * rot;
        self._dirty = true;
    }
}

// TODO implement the rest of the facets
// the main idea here is to construct contiguous areas in memory for different facets
// this is a premature optimization for the Thing/Facet system in general to avoid losing cache
// coherency whilst traversing a series of objects. Probably we want to integrate concurrency
// safety here.
#[derive(Default)]
pub struct WorldFacets {
    pub cameras: Vec<CameraFacet>,
    pub models: Vec<ModelInstanceFacet>,
    pub physical: Vec<PhysicalFacet>,
    pub health: Vec<HealthFacet>,
}

impl WorldFacets {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct World {
    things: Vec<Arc<Mutex<Thing>>>,
    facets: WorldFacets,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start_thing(&mut self) -> ThingBuilder {
        ThingBuilder {
            world: self,
            facets: Vec::new(),
        }
    }

    pub fn get_things(&self) -> &Vec<Arc<Mutex<Thing>>> {
        &self.things
    }

    pub fn get_facets(&mut self) -> &mut WorldFacets {
        &mut self.facets
    }

    pub fn clear(&mut self) {
        self.facets.cameras.clear();
        self.facets.health.clear();
        self.facets.models.clear();
        self.facets.physical.clear();
    }
}

pub struct ThingBuilder<'a> {
    world: &'a mut World,
    facets: Vec<FacetIndex>,
}

impl<'a> ThingBuilder<'a> {
    pub fn with_camera(mut self, camera: CameraFacet) -> Self {
        let idx = self.world.facets.cameras.len();
        self.world.facets.cameras.push(camera);
        self.facets.push(FacetIndex::Camera(idx));
        self
    }

    pub fn with_model(mut self, transform: Matrix4<f32>, model: Arc<model::Model>) -> Self {
        let idx = self.world.facets.models.len();
        self.world
            .facets
            .models
            .push(ModelInstanceFacet { transform, model });
        self.facets.push(FacetIndex::Model(idx));
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
        Thing { id, facets }
    }

    pub fn get_camera_fi(&self) -> Option<FacetIndex> {
        self.facets
            .iter()
            .find(|i| {
                if let FacetIndex::Camera(_) = i {
                    true
                } else {
                    false
                }
            })
            .cloned()
    }

    pub fn get_model_fi(&self) -> Option<FacetIndex> {
        self.facets
            .iter()
            .find(|i| {
                if let FacetIndex::Camera(_) = i {
                    true
                } else {
                    false
                }
            })
            .cloned()
    }
}

impl Identifyable for Thing {
    fn identify(&self) -> u64 {
        self.id
    }
}
