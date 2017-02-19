use cgmath::{ Vector3, InnerSpace };
use super::ship::Ship;
use super::mission::{IFF, OrderType, EventType, MissionEvent};
use rand::random;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum AIStateType {
    NONE_AIMETASTATE,
    DISABLED_AIMETASTATE,
    GETTING_DISTANCE_AIMETASTATE,
    GETTING_ALTITUDE_AIMETASTATE,
    EVASIVE_AIMETASTATE,
    FIRING_AIMETASTATE,
    DISABLING_AIMETASTATE,
    MOVING_AIMETASTATE,
    ESCORTING_AIMETASTATE,
    JUMPING_IN_AIMETASTATE,
    PREPARING_TO_JUMP_AIMETASTATE,
    JUMPING_OUT_AIMETASTATE,
    EXITING_HANGAR_AIMETASTATE,
    ENTERING_HANGAR_AIMETASTATE,
    FLYING_HOME_AIMETASTATE,
    POSITIONING_FOR_BOARDING_AIMETASTATE,
    ORIENTING_FOR_BOARDING_AIMETASTATE,
    BOARDING_AIMETASTATE,
    PATROLLING_FIGHTERS_AIMETASTATE,
    PATROLLING_ALL_AIMETASTATE,
    PATROLLING_BOARDING_AIMETASTATE,
}


pub struct AIMetaState {
    pub state_type: AIStateType,
    position: [Vector3<f32>;3],
    pub extra_data1: i32,
    extra_data2: i32,
    timestamp: u64
}

pub trait AIShip {
    fn step_ai(&self, millis_passed: i32, current_time: u64);
    fn fire_main_gun(&self);
    fn fire_emp_gun(&self);
    fn react_to_fire(&mut self, current_time: u64);
    fn is_enemy(&self, other: &Ship) -> bool;
}


impl AIShip for Ship {

    fn step_ai(&self, millis_passed: i32, current_time: u64) {
        panic!("unimplemented");
    }
    fn fire_main_gun(&self) {
        panic!("unimplemented");
    }
    fn fire_emp_gun(&self) {
        panic!("unimplemented");
    }

    fn react_to_fire(&mut self, current_time: u64) {
        if self.orders.order_type != OrderType::MOVE_ORDERS &&
            ! match self.meta_state.state_type {
                AIStateType::NONE_AIMETASTATE |
                AIStateType::DISABLED_AIMETASTATE |
                AIStateType::ENTERING_HANGAR_AIMETASTATE |
                AIStateType::EXITING_HANGAR_AIMETASTATE |
                AIStateType::JUMPING_IN_AIMETASTATE |
                AIStateType::JUMPING_OUT_AIMETASTATE |
                AIStateType::ORIENTING_FOR_BOARDING_AIMETASTATE |
                AIStateType::POSITIONING_FOR_BOARDING_AIMETASTATE |
                AIStateType::BOARDING_AIMETASTATE => true,
               _ => false
            } {
            if self.last_hit_at + (FIRING_RATE as u64) * 2 > current_time &&
                self.meta_state.state_type != AIStateType::EVASIVE_AIMETASTATE {
                self.meta_state.state_type = AIStateType::EVASIVE_AIMETASTATE;
                self.meta_state.extra_data1 = random::<i32>() % 4;
            }
        }

        if self.last_hit_at == 0 {
            let event = MissionEvent::new(
                EventType::ATTACKED_EVENT_TYPE,
                self.ship_number,
                0,
                Vector3::new(0f32,0f32,0f32),
                self.designation.clone()
            );
            // mission.process_event(event);
        }
        self.last_hit_at = current_time;
    }

    fn is_enemy(&self, target: &Ship) -> bool {
        match target.iff {
            IFF::NEUTRAL_FRIENDLY_IFF => false,
            IFF::FRIENDLY_IFF if self.iff == IFF::ENEMY_IFF => true,
            IFF::ENEMY_IFF if self.iff == IFF::FRIENDLY_IFF => true,
            IFF::NEUTRAL_ENEMY_IFF if self.iff != IFF::NEUTRAL_ENEMY_IFF => true,
            IFF::NEUTRAL_ENEMY_IFF => true,
            _ => false
        }
    }
}

trait AIShipPrivate {
    fn turn_towards(&mut self, target: &Vector3<f32>, millis_passed: u32);
    fn turn_away_from(&mut self, target: &Vector3<f32>, millis_passed: u32);
    fn target_by_name(&mut self, target_designation: String, ship_number: i32, mode: TargetingMode);
    fn fire_auto_guns(&mut self, current_time: u64);
    fn avoid_collisions(&mut self, current_time: u64);
    fn escort_target(&mut self, current_time: u64, millis_passed: u64);
}

fn perfect_shot_direction(world_origin: Vector3<f32>, target: &Ship){panic!("unimplemented");}

impl AIShipPrivate for Ship {
    fn turn_towards(&mut self, target: &Vector3<f32>, millis_passed: u32) {
        let v = target - self.position;
        let normalized = v.normalize();
        let rotation_axis = normalized.cross(*target);
        self.rotate(-Ship::get_ship_type(&self).turning_speed*(millis_passed as f64), rotation_axis);
    }

    fn turn_away_from(&mut self, target: &Vector3<f32>, millis_passed: u32) {
        let v = target - self.position;
        let normalized = v.normalize();
        let rotation_axis = normalized.cross(*target);
        self.rotate(Ship::get_ship_type(&self).turning_speed*(millis_passed as f64), rotation_axis);
    }

    fn target_by_name(&mut self, target_designation: String, ship_number: i32, mode: TargetingMode) {
        panic!("unimplemented");
    }

    fn fire_auto_guns(&mut self, current_time: u64) {
        panic!("unimplemented");
    }

    fn avoid_collisions(&mut self, current_time: u64){
        panic!("unimplemented");
    }

    fn escort_target(&mut self, current_time: u64, millis_passed: u64) {
        panic!("unimplemented");
    }
}

pub fn state_name(state_type: AIStateType) -> &'static str {
    return match state_type {
        AIStateType::NONE_AIMETASTATE => "WAITING",
        AIStateType::DISABLED_AIMETASTATE =>  "DISABLED",
        AIStateType::EVASIVE_AIMETASTATE =>  "EVADING",
        AIStateType::FIRING_AIMETASTATE =>  "ATTACKING",
        AIStateType::DISABLING_AIMETASTATE =>  "DISABLING",
        AIStateType::GETTING_DISTANCE_AIMETASTATE =>  "PREPARING ATTACK",
        AIStateType::GETTING_ALTITUDE_AIMETASTATE =>  "CLIMBING",
        AIStateType::MOVING_AIMETASTATE =>  "MOVING",
        AIStateType::JUMPING_IN_AIMETASTATE =>  "JUMPING IN",
        AIStateType::PREPARING_TO_JUMP_AIMETASTATE =>  "JUMPING OUT",
        AIStateType::JUMPING_OUT_AIMETASTATE =>  "JUMPING OUT",
        AIStateType::FLYING_HOME_AIMETASTATE =>  "FLYING HOME",
        AIStateType::PATROLLING_FIGHTERS_AIMETASTATE =>  "PATROLLING",
        AIStateType::PATROLLING_ALL_AIMETASTATE =>  "PATROLLING",
        AIStateType::PATROLLING_BOARDING_AIMETASTATE =>  "PATROLLING",
        AIStateType::ENTERING_HANGAR_AIMETASTATE =>  "ENTERING HANGAR",
        AIStateType::EXITING_HANGAR_AIMETASTATE =>  "EXITING HANGAR",
        AIStateType::POSITIONING_FOR_BOARDING_AIMETASTATE =>  "BOARDING",
        AIStateType::ORIENTING_FOR_BOARDING_AIMETASTATE =>  "BOARDING",
        AIStateType::BOARDING_AIMETASTATE =>  "BOARDING",
        AIStateType::ESCORTING_AIMETASTATE =>  "ESCORTING",
    }
}


// module private
const FIRING_RATE: i32 = 800;
const BOARDING_DISTANCE: i32 = 20;

#[allow(non_camel_case_types)]
enum TargetingMode {
    ALL_TARGETINGMODE = 0,
    NONDISABLED_TARGETINGMODE,
    ONLYDISABLED_TARGETINGMODE,
}