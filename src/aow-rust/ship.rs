use cgmath::Vector3;

use super::model::Model;
use super::textures::Texture;
use super::mission::{ IFF, VisibleIFF, Orders };
use super::ai::AIMetaState;
use super::game::Laser;

pub const NUMBER_OF_SHIP_TYPES: usize = 18;
pub const MAX_NUMBER_OF_GUNS: usize = 4;
pub const MAX_NUMBER_OF_OBBS: usize = 21;
pub const MAX_NUMBER_OF_AUTOGUNS: usize = 12;
pub const MAX_NUMBER_OF_ENGINES: usize = 5;
pub const BUNCHUP_CHECK_TIME: i32 = 100;
pub const PLAYER_SHIP_ID: i32 = 0;
pub const INVALID_SHIP_ID: i32 = -1;

pub enum ShipClass {
    UNKNOWN_CLASS = -1,
    STARFIGHTER_CLASS = 0,
    CAPITAL_CLASS,
}

pub enum ShipTypeID {
    R44_SHIP_TYPE = 0,
    R47_SHIP_TYPE,
    B82_SHIP_TYPE,
    CORVETTE_SHIP_TYPE,
    FRIGATE_SHIP_TYPE,
    FRIGATE_B_SHIP_TYPE,
    APC_SHIP_TYPE,
    SHIPOFTHELINE_SHIP_TYPE,
    HAULER_SHIP_TYPE,
    STATION_SHIP_TYPE,
    TRANSPORT_SHIP_TYPE,
    CONTAINER_SHIP_TYPE,
    SLOOP_SHIP_TYPE,
    DROP_SHIP_TYPE,
    TRAP_SHIP_TYPE,
    CARRIER_SHIP_TYPE,
    RELAY_SHIP_TYPE,

    ASTEROID1_SHIP_TYPE,
    ASTEROID2_SHIP_TYPE,
}

pub enum ShipStatus {
    NORMAL_SHIP_STATUS = 0,
    DESTROYED_SHIP_STATUS,
    DISABLED_SHIP_STATUS
}

pub enum SubsystemStatus {
    SUBSYSTEM_ACTIVE = 0,
    SUBSYSTEM_DISABLED,
    SUBSYSTEM_DESTROYED,

    SUBSYSTEM_STATUS_COUNT
}

pub enum SubsystemType {
    SUBSYSTEM_TYPE_AUTOGUN = 0,
    SUBSYSTEM_TYPE_SHIELDGENERATOR,
    SUBSYSTEM_TYPE_ENGINE,

    SUBSYSTEM_TYPE_COUN
}

pub struct StaticShipHeader {
    ship_model: Model,
    ship_texture: Texture,
    ship_class: ShipClass,
    type_name: String
}

pub struct ShipSubsystem {
    time_data: u64,
    status: u8,
    ttype: u8,
    structure: u8,
    system: u8,
    shields: u8
}

pub struct StaticShipType {
    pad: u32,
    header: StaticShipHeader,
    size: f64, // data? should it be a size_t?

    number_of_main_guns: u32,
    main_guns: [Vector3<f32>; MAX_NUMBER_OF_GUNS],

    number_of_emp_guns: u32,
    emp_guns: [Vector3<f32>; MAX_NUMBER_OF_GUNS],

    number_of_obbs: u32,
    obbs: [[Vector3<f32>; MAX_NUMBER_OF_OBBS];2],

    number_of_open_obbs: u32,
    open_obbs: [[Vector3<f32>; MAX_NUMBER_OF_OBBS];2],

    number_of_autoguns: u32,
    auto_guns: [[Vector3<f32>; MAX_NUMBER_OF_AUTOGUNS];2],

    number_of_engines: u32,
    engines: [Vector3<f32>; MAX_NUMBER_OF_ENGINES],
    engine_size: [f32; MAX_NUMBER_OF_ENGINES],

    docking_bays:[Vector3<f32>;3], // position, forward, up

    spin_on_death: i32,
    hull_points: i32,
    shield_points: i32,
    system_points: i32,
    regeneration_delay: i32,
    max_velocity: f64,
    pub turning_speed: f64,
    turn_from_at_sqr_distance: f64,
    turn_towards_at_sqr_distance: f64,
}

pub struct Ship {
    ship_type: ShipTypeID,
    pub designation: String,
    pub ship_number: i32,
    ships_in_group: i32,
    visible_iff: VisibleIFF,
    pub position: Vector3<f32>,
    velocity_vector: Vector3<f32>,
    forward: Vector3<f32>,
    up: Vector3<f32>,
    external_force: Vector3<f32>,
    velocity: f64,
    desired_engine_output: f64,
    guns_fired_at: u64,
    auto_gun_time_data: [u64; MAX_NUMBER_OF_AUTOGUNS],
    auto_guns: [ShipSubsystem; MAX_NUMBER_OF_AUTOGUNS],
    player_exit: i32,
    inspected: i32,
    cargo: String,
    hull: i32,
    shields: i32,
    millis_since_regen: i32, //??
    system: i32,
    energy: f32,
    using_afterburners: i32,
    selected_target: i32,
    last_hit_by: i32,
    status: ShipStatus,
    destroyed_at: u64,
    inv_mv_calculated_at: u64,
    inv_mv_matrix: [f32;16],
    pub iff: IFF,
    pub orders: Orders,
    pub meta_state: AIMetaState,
    pub last_hit_at: u64,
    last_avoided_bunching_at: u64,
    skill_level: i32,
}

fn die(){ panic!("Unimplemented"); }

impl Ship {
    //statics
    pub fn get_ship_class(classname: String) -> ShipClass {panic!("Unimplemented"); }
    pub fn get_ship_type(ship: &Ship) -> StaticShipType {panic!("Unimplemented");}
    pub fn get_ship_type_by_id(id:ShipTypeID) -> StaticShipType {panic!("Unimplemented");}

    pub fn create(
        ship_type: ShipTypeID,
        position: Vector3<f32>,
        up: Vector3<f32>,
        direction: Vector3<f32>) -> Self {

        panic!("Unimplemented");

    }

    pub fn get_model_view(&self) -> &[f32;16] {
        panic!("Unimplemented");
    }
    pub fn calculate_inv_mv(&mut self, current_time: u64) { die(); }
    pub fn match_speed_with_target(&self) -> f32 { die(); 0f32 }
    pub fn pitch(&self, angle: f64) {die();}
    pub fn yaw(&self, angle: f64) {die();}
    pub fn roll(&self, angle: f64) {die();}
    pub fn rotate(&self, angle: f64, around: Vector3<f32>) {die();}
    pub fn step(&self, millis_passed: u64, current_time: u64) {die();}
    pub fn take_weapon_damage(&self, laser: Laser) {die();}
    pub fn take_damage(&self, dmg_type: u8, damage: i32) {die();}
    pub fn apply_drag(&self){ die(); }
}