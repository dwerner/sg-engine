/*******************************************************************************
* Copyright 2015 Neil Petrick
*
* This file is part of Assembly of Worlds.
*
* Assembly of Worlds is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* Assembly of Worlds is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with Assembly of Worlds.  If not, see <http://www.gnu.org/licenses/>.
*
*****************************************************************************/

pub enum Texture {
    B82COCKPIT_TEX = 0,
    R44COCKPIT_TEX,
    R47COCKPIT_TEX,
    CONSOLE_TEX,
    THROTTLE_TEX,

    LOGOTYPE_TEX,

    R44_SHIP_TEX,
    R47_SHIP_TEX,
    B82_SHIP_TEX,
    CORVETTE_SHIP_TEX,
    FRIGATE_SHIP_TEX,
    FRIGATE_B_SHIP_TEX,
    APC_SHIP_TEX,
    SOTL_SHIP_TEX,
    HAULER_SHIP_TEX,
    STATION_SHIP_TEX,
    TRANSPORT_SHIP_TEX,
    CONTAINER_SHIP_TEX,
    SLOOP_SHIP_TEX,
    DROP_SHIP_TEX,
    TRAP_SHIP_TEX,
    CARRIER_SHIP_TEX,
    RELAY_SHIP_TEX,
    ASTEROID_SHIP_TEX,

    // Thumbs always arrayed green, yellow, red
    R44_THUMB_GREEN_TEX,
    R44_THUMB_YELLOW_TEX,
    R44_THUMB_RED_TEX,

    R47_THUMB_GREEN_TEX,
    R47_THUMB_YELLOW_TEX,
    R47_THUMB_RED_TEX,

    APC_THUMB_GREEN_TEX,
    APC_THUMB_YELLOW_TEX,
    APC_THUMB_RED_TEX,

    CRV_THUMB_GREEN_TEX,
    CRV_THUMB_YELLOW_TEX,
    CRV_THUMB_RED_TEX,

    FRG_THUMB_GREEN_TEX,
    FRG_THUMB_YELLOW_TEX,
    FRG_THUMB_RED_TEX,

    SOTL_THUMB_GREEN_TEX,
    SOTL_THUMB_YELLOW_TEX,
    SOTL_THUMB_RED_TEX,

    HAUL_THUMB_GREEN_TEX,
    HAUL_THUMB_YELLOW_TEX,
    HAUL_THUMB_RED_TEX,

    STAT_THUMB_GREEN_TEX,
    STAT_THUMB_YELLOW_TEX,
    STAT_THUMB_RED_TEX,

    TRN_THUMB_GREEN_TEX,
    TRN_THUMB_YELLOW_TEX,
    TRN_THUMB_RED_TEX,

    CNT_THUMB_GREEN_TEX,
    CNT_THUMB_YELLOW_TEX,
    CNT_THUMB_RED_TEX,

    SLP_THUMB_GREEN_TEX,
    SLP_THUMB_YELLOW_TEX,
    SLP_THUMB_RED_TEX,

    DROP_THUMB_GREEN_TEX,
    DROP_THUMB_YELLOW_TEX,
    DROP_THUMB_RED_TEX,

    TRAP_THUMB_GREEN_TEX,
    TRAP_THUMB_YELLOW_TEX,
    TRAP_THUMB_RED_TEX,

    CARRIER_THUMB_GREEN_TEX,
    CARRIER_THUMB_YELLOW_TEX,
    CARRIER_THUMB_RED_TEX,

    RELAY_THUMB_GREEN_TEX,
    RELAY_THUMB_YELLOW_TEX,
    RELAY_THUMB_RED_TEX,

    B82_THUMB_GREEN_TEX,
    B82_THUMB_YELLOW_TEX,
    B82_THUMB_RED_TEX,

    AST_THUMB_GREEN_TEX,
    AST_THUMB_YELLOW_TEX,
    AST_THUMB_RED_TEX,

    PLANET_THUMB_TEX,

    FONTWHITE_TEX,

    FONTLIGHTGREEN_TEX,
    FONTLIGHTYELLOW_TEX,
    FONTLIGHTRED_TEX,

    FONTDARKGREEN_TEX,
    FONTDARKYELLOW_TEX,
    FONTDARKRED_TEX,

    PINGGREEN_TEX,
    PINGYELLOW_TEX,
    PINGRED_TEX,

    EXPLOSION_TEX,

    CROSSHAIR_TEX,
    CROSSHAIR_DUAL_TEX,
    CROSSHAIR_QUAD_TEX,
    CROSSHAIR_QUAD_TOP_TEX,
    EXPECTED_HIT_TEX,
    ARROW_TEX,
    TARGET_TEX,

    ENGINE_TEX,
    LASER_TEX,
    EMP_TEX,

    TEXTCONSOLE_TEX,

    STEERINGPAD_TEX,
    BUTTONBACK_TEX,

    SPACE_FRONT_TEX,
    SPACE_BACK_TEX,
    SPACE_LEFT_TEX,
    SPACE_RIGHT_TEX,
    SPACE_TOP_TEX,
    SPACE_BOTTOM_TEX,

    SPACE_RIGHT_PLANET1_TEX,
    SPACE_RIGHT_PLANET2_TEX,
    SPACE_RIGHT_PLANET3_TEX,
    SPACE_RIGHT_PLANET4_TEX,

    ATMOSPHERE_FRONT_TEX,
    ATMOSPHERE_BACK_TEX,
    ATMOSPHERE_LEFT_TEX,
    ATMOSPHERE_RIGHT_TEX,
    ATMOSPHERE_TOP_TEX,
    ATMOSPHERE_BOTTOM_TEX,

    GROUND_TEX,
    WATER_TEX,

    NUMBER_OF_TEXTURES
}

pub fn initializeTextures(){ }
pub fn uninitializeTextures(){ }

pub fn getTexture(textureName: &str) -> Texture {Texture::TEXTCONSOLE_TEX}
pub fn getTextureHandle(texture: Texture) -> u32 /*GLuint*/ { 42 }
