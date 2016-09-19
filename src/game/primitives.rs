use cgmath::{Vector3};
use std::vec::Vec;

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct RegionId(pub u32);

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct ObjectId(pub u32);

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Game {
	regions: Vec<Region>
}
impl Game {
	pub fn new(regions: Vec<Region>) -> Self {
		Game{ regions: regions }
	}

	pub fn add_region(&mut self, region: Region) {
		self.regions.push(region);
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Region {
	id: RegionId,
	last_update: u64, 
	game_objects: Vec<GameObject>
}

impl Region {
	pub fn new(id: RegionId) -> Self {
		Region{
			id: id,
			last_update: 0,
			game_objects: Vec::new()
		}
	}
	pub fn add_game_object(&mut self, game_object: GameObject) {
		self.game_objects.push(game_object);
	}
	#[inline] pub fn get_id(&self) -> &RegionId { &self.id }
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
pub struct GameObject {
	id: ObjectId,
	position: Vector3<f32>,
	rotation: Vector3<f32>,
	scale:    Vector3<f32>,
}

impl GameObject {

	pub fn transit_to_region(game_object: ObjectId, source: &mut Region, dest: &mut Region) {
		
	}

	pub fn new(id: ObjectId, p: Vector3<f32>, r: Vector3<f32>, s: Vector3<f32>) -> Self {
		GameObject{ 
			id: id,
			position: p,
			rotation: r,
			scale:		s
		}
	}

	#[inline] pub fn get_id(&self) -> &ObjectId { &self.id }
}



