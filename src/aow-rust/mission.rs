use cgmath::Vector3;
use super::ship::Ship;
use std::cell::Cell;

const MAX_NUMBER_OF_DEPLOYMENTS: usize = 50;
const MAX_NUMBER_OF_GOALS: usize = 20;
const MAX_NUMBER_OF_RESPONSES: usize = 30;
const MAX_NUMBER_OF_EVENTS_IN_RESPONSE: usize = 8;
const MAX_NUMBER_OF_MISSION_SOUNDS: usize = 40;
const MAX_NUMBER_OF_TIME_MARKERS: usize = 32;
const MAX_NUMBER_OF_SHIPS_IN_GROUP: usize = 30;

/*******************************************************************************
 * Local types
 *
 ******************************************************************************/
#[allow(non_camel_case_types)]
enum EventResponseType {
    NO_RESPONSE_EVENT_RESPONSE_TYPE = 0,
    DEPLOY_EVENT_RESPONSE_TYPE,
    RESTORE_SYSTEMS_EVENT_RESPONSE_TYPE,
    LOWER_SHIELDS_EVENT_RESPONSE_TYPE,
    DESTROY_EVENT_RESPONSE_TYPE,
    CHANGE_ORDERS_EVENT_RESPONSE_TYPE,
    CHANGE_ORDERS_SPECIFIC_EVENT_RESPONSE_TYPE,
    CHANGE_CARGO_EVENT_RESPONSE_TYPE,
    CHANGE_IFF_EVENT_RESPONSE_TYPE,
    PLAY_VOICE_EVENT_RESPONSE_TYPE,
    SET_TIME_MARKER_EVENT_RESPONSE_TYPE,
    SET_UI_ELEMENT_STATE_EVENT_RESPONSE_TYPE, // used in tutorial
}

#[allow(non_camel_case_types)]
enum DeploymentType {
    MATERIALIZE_DEPLOYMENT_TYPE = 0,
    HYPERSPACE_DEPLOYMENT_TYPE,
    DOCKINGBAY_DEPLOYMENT_TYPE
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GoalStatus {
    UNDECIDED_GOAL_STATUS = 0,
    FAIL_GOAL_STATUS,
    PASS_GOAL_STATUS,
}

#[allow(non_camel_case_types)]
enum EventResponseClass {
    LOGICAL_AND_RESPONSECLASS = 0,
    LOGICAL_OR_RESPONSECLASS,
}

#[allow(non_camel_case_types)]
pub enum MissionStatus {
    UNDECIDED_MISSION_STATUS = 0,
    FAIL_MISSION_STATUS,
    PASS_MISSION_STATUS,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum IFF {
    FRIENDLY_IFF = 0,
    ENEMY_IFF,
    NEUTRAL_ENEMY_IFF,
    NEUTRAL_FRIENDLY_IFF,
}

#[allow(non_camel_case_types)]
pub enum VisibleIFF {
    DEFAULT_IFF = 0,
    GREEN_IFF,
    YELLOW_IFF,
    RED_IFF
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum OrderType {
    WAIT_ORDERS = 0, // IDLE
    ATTACK_ORDERS,
    DISABLE_ORDERS,
    MOVE_ORDERS,
    ESCORT_ORDERS,
    PATROL_FIGHTERS_ORDERS,
    PATROL_ALL_ORDERS,
    PATROL_BOARD_ORDERS,
    DOCK_ORDERS,
    JUMP_ORDERS,
    BOARD_ORDERS
}

pub struct Orders {
    pub order_type: OrderType,
    designation: String,
    position: [Vector3<f32>;3],
    extra_data1: i32,
    extra_data2: i32
}

#[derive(PartialEq, Eq, Clone)]
pub enum EventType {
    NO_EVENT_TYPE = 0,
    MISSION_START_EVENT_TYPE,
    TIMED_EVENT_TYPE,
    MARKED_TIMED_EVENT_TYPE,
    DESTROYED_EVENT_TYPE,
    ALL_DESTROYED_EVENT_TYPE,
    DISABLED_EVENT_TYPE,
    ALL_DISABLED_EVENT_TYPE,
    BOARDED_EVENT_TYPE,
    ALL_BOARDED_EVENT_TYPE,
    INSPECTED_EVENT_TYPE,
    ALL_INSPECTED_EVENT_TYPE,
    LEFT_AREA_EVENT_TYPE,
    ALL_LEFT_AREA_EVENT_TYPE,
    HALF_LEFT_AREA_EVENT_TYPE, // ADD FOR THE OTHERS WHEN NEEDED
    LOWSHIELDS_EVENT_TYPE,
    NOSHIELDS_EVENT_TYPE,
    REMOVED_EVENT_TYPE,
    ALL_REMOVED_EVENT_TYPE,
    ALL_NEUTRALIZED_EVENT_TYPE,
    ARRIVED_AT_DESTINATION_EVENT_TYPE,
    MISSION_FAILED_EVENT_TYPE,
    MISSION_SUCCEEDED_EVENT_TYPE,
    ATTACKED_EVENT_TYPE,
    PLAYER_CHANGED_TARGET_EVENT_TYPE, // Used for tutorial. Only triggers on next/previous
}



pub struct MissionEvent {
    event_type: EventType,
    extra_data1: i32,
    extra_data2: i32,
    position: Vector3<f32>,
    designation: String,
}

impl MissionEvent {
    pub fn new(
        event_type: EventType,
        extra_data1: i32,
        extra_data2: i32,
        position: Vector3<f32>,
        designation: String
    ) -> Self {
        MissionEvent {
            event_type: event_type,
            extra_data1: extra_data1,
            extra_data2: extra_data2,
            position: position,
            designation: designation
        }
    }
}

pub enum EnvironmentType {
    OPEN_SPACE_ENVIRONMENT = 0,
    BLUE_PLANET_ENVIRONMENT,
    RED_PLANET_ENVIRONMENT,
    YELLOW_PLANET_ENVIRONMENT,
    GREEN_PLANET_ENVIRONMENT,
    ATMOSPHERE_ENVIRONMENT,
}

pub struct Deployment {
    pub designation: String,
    pub number_of_ships: u32,
    pub ships_destroyed: u32
}


#[allow(non_camel_case_types)]
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum GoalType {
    SURVIVE_GOAL_TYPE,
    ALL_SURVIVE_GOAL_TYPE,
    DESTROYED_GOAL_TYPE,
    ALL_DESTROYED_GOAL_TYPE,
    DISABLED_GOAL_TYPE,
    ALL_DISABLED_GOAL_TYPE,
    BOARDED_GOAL_TYPE,

    ALL_BOARDED_GOAL_TYPE,

    NEUTRALIZED_GOAL_TYPE,
    ALL_NEUTRALIZED_GOAL_TYPE,
    DEPART_GOAL_TYPE,
    ALL_DEPART_GOAL_TYPE,
    HALF_DEPART_GOAL_TYPE, // ADD FOR THE OTHERS WHEN NEEDED
    INSPECT_GOAL_TYPE,
    ALL_INSPECT_GOAL_TYPE,
    REMOVED_GOAL_TYPE,
    ALL_REMOVED_GOAL_TYPE,
}

pub trait GoalHandler {
    fn process_event(&self, goal: &Goal, event: MissionEvent);
}

//#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Goal {
    pub designation: String,
    pub goal_type: GoalType,
    pub extra_data1: i32, // smelly. but yeah
    pub extra_data2: i32,
    pub handler: Box<GoalHandler>,
    pub status: Cell<GoalStatus>, // goalstatus needs to impl Copy+Clone now
}

impl Goal {
    pub fn fail(&self) { self.status.set(GoalStatus::FAIL_GOAL_STATUS); }
    pub fn pass(&self) { self.status.set(GoalStatus::PASS_GOAL_STATUS); }
}

pub struct SurviveGoal;
impl GoalHandler for SurviveGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        match event.event_type {
            EventType::DESTROYED_EVENT_TYPE => if event.designation == goal.designation {
                if event.extra_data1 == goal.extra_data1 &&
                    goal.extra_data1 != -1 {
                    goal.fail();
                }
            },
            EventType::ALL_DESTROYED_EVENT_TYPE => if event.designation == goal.designation {
                goal.fail();
            },
            _ => {}
        }
    }
}

pub struct AllSurviveGoal;
impl GoalHandler for AllSurviveGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.event_type == EventType::DESTROYED_EVENT_TYPE &&
            event.designation == goal.designation {
            goal.fail();
        }
    }
}

pub struct NeutralizedGoal;
impl GoalHandler for NeutralizedGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        match event.event_type {
            EventType::LEFT_AREA_EVENT_TYPE => {
                if event.designation == goal.designation &&
                    event.extra_data1 == goal.extra_data1 {
                    goal.fail();
                }
            },
            EventType::DESTROYED_EVENT_TYPE |
            EventType::DISABLED_EVENT_TYPE => {
                if event.designation == goal.designation &&
                    event.extra_data1 == goal.extra_data1 {
                    goal.pass();
                }
            }
            _ => {}
        }
    }
}

pub struct AllNeutralizedGoal;
impl GoalHandler for AllNeutralizedGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.designation == goal.designation {
            if event.event_type == EventType::LEFT_AREA_EVENT_TYPE {
                goal.fail();
            } else if event.event_type == EventType::ALL_NEUTRALIZED_EVENT_TYPE {
                goal.pass();
            }
        }
    }
}

pub struct DepartGoal;
impl GoalHandler for DepartGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.designation == goal.designation {
            if event.event_type == EventType::LEFT_AREA_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                goal.pass();
            } else if event.event_type == EventType::DESTROYED_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                goal.fail();
            } else if event.event_type == EventType::ALL_DESTROYED_EVENT_TYPE {
                goal.fail();
            }
        }
    }
}

pub struct AllDepartGoal;
impl GoalHandler for AllDepartGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.designation == goal.designation {
            if event.event_type == EventType::LEFT_AREA_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                goal.pass();
            } else if event.event_type == EventType::DESTROYED_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                goal.fail();
            } else if event.event_type == EventType::ALL_DESTROYED_EVENT_TYPE {
                goal.fail();
            }
        }
    }
}

pub struct HalfDepartGoal <'a> {
    pub mission: &'a Box<Mission>
}
impl <'a> GoalHandler for HalfDepartGoal <'a> {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.designation == goal.designation {
            if event.event_type == EventType::HALF_LEFT_AREA_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                goal.pass();
            } else if event.event_type == EventType::DESTROYED_EVENT_TYPE &&
                event.extra_data1 == goal.extra_data1 {
                if let Some(deployment) = self.mission.deployments.iter(). find(|i| {
                        i.designation == event.designation
                }) {
                    if deployment.ships_destroyed > deployment.number_of_ships / 2 {
                        goal.fail();
                    }
                }
            }
        }
    }
}

pub struct InspectGoal;
impl GoalHandler for InspectGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if goal.status.get() == GoalStatus::UNDECIDED_GOAL_STATUS {
            if event.designation == goal.designation {
                match event.event_type {
                    EventType::LEFT_AREA_EVENT_TYPE |
                    EventType::DESTROYED_EVENT_TYPE if event.extra_data1 == goal.extra_data1 => {
                        goal.fail();
                    }
                    EventType::INSPECTED_EVENT_TYPE if (event.extra_data1 == goal.extra_data1 || goal.extra_data1 == -1) => {
                        goal.pass();
                    }
                    EventType::ALL_DESTROYED_EVENT_TYPE |
                    EventType::ALL_LEFT_AREA_EVENT_TYPE if event.designation == goal.designation => {
                        goal.fail();
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct AllInspectGoal <'a> {
    pub mission: &'a Box<Mission>
}
impl <'a> GoalHandler for AllInspectGoal <'a> {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if event.designation == goal.designation {
            match event.event_type {
                EventType::LEFT_AREA_EVENT_TYPE |
                EventType::DESTROYED_EVENT_TYPE if event.extra_data1 == goal.extra_data1 => {
                    goal.fail();
                }
                EventType::INSPECTED_EVENT_TYPE if (event.extra_data1 == goal.extra_data1 || goal.extra_data1 == -1) => {
                    goal.pass();
                }
                EventType::ALL_DESTROYED_EVENT_TYPE |
                EventType::ALL_LEFT_AREA_EVENT_TYPE if event.designation == goal.designation => {
                    goal.fail();
                }
                _ => {}
            }
        }
    }
}

pub struct BoardedGoal{ }
impl GoalHandler for BoardedGoal {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        if goal.status.get() == GoalStatus::UNDECIDED_GOAL_STATUS {
            if event.designation == goal.designation {
                match event.event_type {
                    EventType::LEFT_AREA_EVENT_TYPE |
                    EventType::DESTROYED_EVENT_TYPE if event.extra_data1 == goal.extra_data1 => {
                        goal.fail();
                    }
                    EventType::BOARDED_EVENT_TYPE if (event.extra_data1 == goal.extra_data1 || goal.extra_data1 == -1) => {
                        goal.pass();
                    }
                    EventType::ALL_DESTROYED_EVENT_TYPE |
                    EventType::ALL_LEFT_AREA_EVENT_TYPE if event.designation == goal.designation => {
                        goal.fail();
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct AllBoardedGoal <'a> {
    pub mission: &'a Box<Mission>
}
impl <'a> GoalHandler for AllBoardedGoal <'a> {
    fn process_event(&self, goal: &Goal, event: MissionEvent) {
        match event.event_type {
            EventType::ALL_BOARDED_EVENT_TYPE => {
                goal.pass();
            }
            EventType::LEFT_AREA_EVENT_TYPE |
            EventType::DESTROYED_EVENT_TYPE if event.designation == goal.designation => {
                if let Some(deployment) = self.mission.deployments.iter().find(|i| { i.designation == event.designation }) {
                    goal.fail();
                }
            }
            EventType::BOARDED_EVENT_TYPE if (event.extra_data1 == goal.extra_data1 || goal.extra_data1 == -1) => {
                goal.pass();
            }
            EventType::ALL_DESTROYED_EVENT_TYPE |
            EventType::ALL_LEFT_AREA_EVENT_TYPE if event.designation == goal.designation => {
                goal.fail();
            }
            _ => {}
        }
    }
}

impl Goal {
    fn process_event(&self, event: MissionEvent) {
        self.handler.process_event(self, event);
    }
}


pub struct Mission {
    last_time: u64,
    environment_type: EnvironmentType,
    terrain: String,
    terrain_scale: i32,

    number_of_deployments: i32,
    deployments: [Deployment;MAX_NUMBER_OF_DEPLOYMENTS],

    pub number_of_goals: usize,
    pub goals: [Goal; MAX_NUMBER_OF_GOALS],

    mission_status: MissionStatus,
    time_markers: [u64; MAX_NUMBER_OF_TIME_MARKERS]
}


impl Mission {
    fn new() -> Self {
        panic!("unimplemented");
    }
    pub fn load() -> Box<Self> {
        let mut mission = Mission::new();

        // TODO parse and load mission file
        panic!("unimplemented");

        // move us onto the heap
        Box::new(mission)
    }
}

/*******************************************************************************
 * Global functions
 */

pub fn load_mission(filename: &'static str) {

}

pub fn unload_mission() {

}

pub fn start_mission() {

}

pub fn step_mission() {

}

pub fn get_mission_status() -> MissionStatus {
    panic!("unimplemented");
}

pub fn get_environment() -> EnvironmentType {
    panic!("unimplemented");

}

pub fn mission_uses_terrain() -> bool {
    panic!("unimplemented");

}

pub fn render_goals() {

}

pub fn ship_destroyed(ship: &Ship) {

}

pub fn ship_removed(ship: &Ship) {

}

pub fn ship_disabled(ship: &Ship) {

}

pub fn ship_left_area(ship: &Ship) {

}

pub fn ship_boarded(ship: &Ship) {

}

pub fn ship_inspected(ship: &Ship) {

}


