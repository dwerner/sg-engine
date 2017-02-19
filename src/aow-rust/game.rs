use cgmath::Vector3;
use super::mission::{ VisibleIFF };
use super::ship::Ship;
use super::types::{ LaserType, DamageType, GameStateMode };

pub const MAX_NUMBER_OF_SHIPS: usize = 50;
pub const MAX_NUMBER_OF_LASERS: usize = 200;
pub const JUMP_IN_TIME: i32 = 7000;
pub const CANCEL_QUESTION_TIME: i32 = 3000;
pub const LASER_SPEED: f32 = 0.5;
pub const INSPECT_TIME: i32 = 2500;

pub struct Laser {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    laser_type: LaserType,
    damage_type: DamageType,
    time_to_live: i32,
    source_ship: i32,
    target_ship: i32
}

pub struct LogEntry {
    text:String,
    iff: VisibleIFF,
    time: u64,
    previous: Box<LogEntry>
}

#[derive(Default)]
pub struct Statistics {
    plasma_shots: i32,
    plasma_hits: i32,
    emp_shots: i32,
    emp_hits: i32,
    kills: i32,
    disables: i32,
    hits_taken: i32,
    ships_inspected: i32
}

pub struct GameState {
    number_of_ships: u32,
    ships: [Ship; MAX_NUMBER_OF_SHIPS],
    number_of_lasers: u32,
    lasers: [Laser; MAX_NUMBER_OF_LASERS],
    stored_target: i32,
    log: Box<LogEntry>,
    exit_jump_allowed: i32,
    exit_vector: Vector3<f32>,
    end_game:i32,
    game_start: u64,
    last_update_at: u64,
    pause_time: u64,
    inspecting: i32,
    inspect_started_at: u64,
    cancel_requested_at: u64,
    statistics: Statistics,
    state: GameStateMode
}

impl GameState {
    pub fn create( mission: String ) -> Self {
        panic!("Unimplemented");
    }
    pub fn get_game_time() -> u64 {panic!("Unimplemented"); 0 }
    pub fn get_absolute_milliseconds() -> u64 {panic!("Unimplemented"); 0 }
    pub fn add_to_log( text: String ) {panic!("Unimplemented");}
}

