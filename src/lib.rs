#![allow(non_snake_case)]

#[cfg(not(feature = "consts_only"))]
mod modules {
    use smash::app::BattleObjectModuleAccessor;
    pub enum ParamType {
        Common,
        Shared,
        Agent
    }

    extern "Rust" {
        #[link_name = "VarModule__get_int"]
        fn VarModule__get_int(boma: *mut BattleObjectModuleAccessor, what: i32) -> i32;
        #[link_name = "VarModule__get_int64"]
        fn VarModule__get_int64(boma: *mut BattleObjectModuleAccessor, what: i32) -> u64;
        #[link_name = "VarModule__get_float"]
        fn VarModule__get_float(boma: *mut BattleObjectModuleAccessor, what: i32) -> f32;
        #[link_name = "VarModule__is_flag"]
        fn VarModule__is_flag(boma: *mut BattleObjectModuleAccessor, what: i32) -> bool;

        #[link_name = "VarModule__set_int"]
        fn VarModule__set_int(boma: *mut BattleObjectModuleAccessor, what: i32, value: i32);
        #[link_name = "VarModule__set_int64"]
        fn VarModule__set_int64(boma: *mut BattleObjectModuleAccessor, what: i32, value: u64);
        #[link_name = "VarModule__set_float"]
        fn VarModule__set_float(boma: *mut BattleObjectModuleAccessor, what: i32, value: f32);
        #[link_name = "VarModule__set_flag"]
        fn VarModule__set_flag(boma: *mut BattleObjectModuleAccessor, what: i32, value: bool);
        #[link_name = "VarModule__on_flag"]
        fn VarModule__on_flag(boma: *mut BattleObjectModuleAccessor, what: i32);
        #[link_name = "VarModule__off_flag"]
        fn VarModule__off_flag(boma: *mut BattleObjectModuleAccessor, what: i32);

        #[link_name = "VarModule__countdown_int"]
        fn VarModule__countdown_int(boma: *mut BattleObjectModuleAccessor, what: i32, min: i32) -> bool;
        #[link_name = "VarModule__reset"]
        fn VarModule__reset(boma: *mut BattleObjectModuleAccessor, reset_mask: u8);


        #[link_name = "ParamModule__get_int"]
        fn ParamModule__get_int(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> i32;
        #[link_name = "ParamModule__get_int64"]
        fn ParamModule__get_int64(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> u64;
        #[link_name = "ParamModule__get_float"]
        fn ParamModule__get_float(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> f32;
        #[link_name = "ParamModule__get_flag"]
        fn ParamModule__get_flag(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> bool;
    }


    pub mod VarModule {
        use super::*;

        pub const RESET_COMMON_INT:    u8 = 0b00000001;
        pub const RESET_COMMON_INT64:  u8 = 0b00000010;
        pub const RESET_COMMON_FLOAT:  u8 = 0b00000100;
        pub const RESET_COMMON_FLAG:   u8 = 0b00001000;

        pub const RESET_FIGHTER_INT:   u8 = 0b00010000;
        pub const RESET_FIGHTER_INT64: u8 = 0b00100000;
        pub const RESET_FIGHTER_FLOAT: u8 = 0b01000000;
        pub const RESET_FIGHTER_FLAG:  u8 = 0b10000000;

        pub const RESET_COMMON:  u8 = 0x0F;
        pub const RESET_FIGHTER: u8 = 0xF0;
        pub const RESET_ALL:     u8 = 0xFF;

        pub fn get_int(boma: *mut BattleObjectModuleAccessor, what: i32) -> i32 {
            unsafe {
                VarModule__get_int(boma, what)
            }
        }

        pub fn get_int64(boma: *mut BattleObjectModuleAccessor, what: i32) -> u64 {
            unsafe {
                VarModule__get_int64(boma, what)
            }
        }

        pub fn get_float(boma: *mut BattleObjectModuleAccessor, what: i32) -> f32 {
            unsafe {
                VarModule__get_float(boma, what)
            }
        }

        pub fn is_flag(boma: *mut BattleObjectModuleAccessor, what: i32) -> bool {
            unsafe {
                VarModule__is_flag(boma, what)
            }
        }

        pub fn set_int(boma: *mut BattleObjectModuleAccessor, what: i32, value: i32) {
            unsafe {
                VarModule__set_int(boma, what, value)
            }
        }

        pub fn set_int64(boma: *mut BattleObjectModuleAccessor, what: i32, value: u64) {
            unsafe {
                VarModule__set_int64(boma, what, value)
            }
        }

        pub fn set_float(boma: *mut BattleObjectModuleAccessor, what: i32, value: f32) {
            unsafe {
                VarModule__set_float(boma, what, value)
            }
        }

        pub fn set_flag(boma: *mut BattleObjectModuleAccessor, what: i32, value: bool) {
            unsafe {
                VarModule__set_flag(boma, what, value)
            }
        }

        pub fn on_flag(boma: *mut BattleObjectModuleAccessor, what: i32) {
            unsafe {
                VarModule__on_flag(boma, what)
            }
        }

        pub fn off_flag(boma: *mut BattleObjectModuleAccessor, what: i32) {
            unsafe {
                VarModule__off_flag(boma, what)
            }
        }

        pub fn countdown_int(boma: *mut BattleObjectModuleAccessor, what: i32, min: i32) -> bool {
            unsafe {
                VarModule__countdown_int(boma, what, min)
            }
        }

        pub fn reset(boma: *mut BattleObjectModuleAccessor, reset_mask: u8) {
            unsafe {
                VarModule__reset(boma, reset_mask)
            }
        }
    }

    pub mod ParamModule {
        use super::*;

        pub fn get_int(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> i32 {
            unsafe {
                ParamModule__get_int(boma, ty, string)
            }
        }

        pub fn get_int64(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> u64 {
            unsafe {
                ParamModule__get_int64(boma, ty, string)
            }
        }

        pub fn get_float(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> f32 {
            unsafe {
                ParamModule__get_float(boma, ty, string)
            }
        }

        pub fn get_flag(boma: *mut BattleObjectModuleAccessor, ty: ParamType, string: &str) -> bool {
            unsafe {
                ParamModule__get_flag(boma, ty, string)
            }
        }
    }
}
#[cfg(feature = "consts_only")]
mod modules {}
pub use modules::*;

pub mod consts {

    // common consts are allowed to be 0x000 - 0xFFF
    pub mod common {
        // int consts
        pub const COSTUME_SLOT_NUMBER:        i32 = 0x0;
        pub const FIGHTER_KIND:               i32 = 0x1;
        pub const SPECIAL_TURNAROUND_FRAME:   i32 = 0x2;
        pub const FLOAT_TIMER:                i32 = 0x3;
        pub const FLOAT_DURATION:             i32 = 0x4;
        pub const FLOAT_STYLE:                i32 = 0x5;
        pub const MOONWALK_STATE:             i32 = 0x6;
        pub const MAGIC_REPEAT_NUM_HI:        i32 = 0x7;
        pub const MAGIC_REPEAT_NUM_LW:        i32 = 0x8;
        pub const METER_COUNT:                i32 = 0x9;
        pub const METER_LEVEL:                i32 = 0xA;
        pub const METER_TIMER:                i32 = 0xB;
        pub const METER_TO_GAIN:              i32 = 0xC;
        pub const METER_EX_SPECIAL_SCRIPTING: i32 = 0xD;
        pub const GIMMICK_TIMER:              i32 = 0xE;
        pub const GIMMICK_GLOW_TIMER:         i32 = 0xF;
        pub const AIR_ESCAPE_MAGNET_FRAME:    i32 = 0x10;
        pub const DAMAGE_FLY_CANCEL_FRAME:    i32 = 0x11;
        pub const ITEM_THROW_FRAME:           i32 = 0x12;
        pub const ATTACK_DASH_CANCEL_FRAME:   i32 = 0x13;

        // flag consts
        pub const SPECIAL_IS_REVERSED:          i32 = 0x0;
        pub const SPECIAL_ENABLE_TURNAROUND:    i32 = 0x1;
        pub const IS_DITCIT_SLIDING:            i32 = 0x2;
        pub const UP_SPECIAL_CANCEL:            i32 = 0x3;
        pub const SIDE_SPECIAL_CANCEL:          i32 = 0x4;
        pub const UP_SPECIAL_CANCEL_GROUNDED:   i32 = 0x5;
        pub const UP_SPECIAL_INTERRUPT:         i32 = 0x6;
        pub const UP_SPECIAL_INTERRUPT_AIR:     i32 = 0x7;
        pub const SPECIAL_STALL:                i32 = 0x8;
        pub const SPECIAL_STALL_USED:           i32 = 0x9;
        pub const SPECIAL_TURNAROUND:           i32 = 0xA;
        pub const SPECIAL_REVERSED:             i32 = 0xB;
        pub const UNIQ_PROJECTILE_SPAWNED:      i32 = 0xC;
        pub const IRAR_WINDOW:                  i32 = 0xD;
        pub const IRAR_JUMPSQUAT:               i32 = 0xE;
        pub const FLOAT_PAUSE_AERIAL:           i32 = 0xF;
        pub const AERIAL_NO_FLOAT:              i32 = 0x10;
        pub const IS_MOONWALK:                  i32 = 0x11;
        pub const IS_MOONKWALK_JUMP:            i32 = 0x12;
        pub const MAGIC_JAB_DA_CHECK:           i32 = 0x13;
        pub const MAGIC_TILT_CHECK:             i32 = 0x14;
        pub const MAGIC_SMASH_CHECK:            i32 = 0x15;
        pub const MAGIC_AERIAL_CHECK:           i32 = 0x16;
        pub const MAGIC_SPECIAL_CHECK:          i32 = 0x17;
        pub const MAGIC_FADC_CHECK:             i32 = 0x18;
        pub const MAGIC_ATTACK_BOUNCED:         i32 = 0x19;
        pub const MAGIC_CAN_CANCEL:             i32 = 0x1A;
        pub const MAGIC_CANCEL:                 i32 = 0x1B;
        pub const MAGIC_ADDITIONAL_CANCEL:      i32 = 0x1C;
        pub const MAGIC_COMBO_SCALING:          i32 = 0x1C;
        pub const MAGIC_REPEAT_INCREMENT:       i32 = 0x1D;
        pub const LEDGE_TETHER_REWIND:          i32 = 0x1E;
        pub const LEDGE_TETHER_HOGGED:          i32 = 0x1F;
        pub const FOOTSTOOL_AIR_ESCAPE_LOCKOUT: i32 = 0x20;
        pub const METER_USED:                   i32 = 0x21;
        pub const METER_IS_GAIN:                i32 = 0x22;
        pub const METER_EX_SPECIAL:             i32 = 0x23;
        pub const TUMBLE_IS_TUMBLE_KB:          i32 = 0x24;
        pub const TUMBLE_CAN_ESCAPE:            i32 = 0x25;
        pub const IS_HEAVY_AERIAL:              i32 = 0x26;
        pub const ENABLE_AIR_ESCAPE_JUMPSQUAT:  i32 = 0x27;
        pub const ENABLE_AIR_ESCAPE_MAGNET:     i32 = 0x28;
        pub const ENABLE_DAMAGE_FLY_CANCEL:     i32 = 0x29;
        pub const DISABLE_DAMAGE_FLY_CANCEL:    i32 = 0x2A;
        pub const DAMAGE_FLY_CANCEL_ANGLE_CHK:  i32 = 0x2B;
        pub const ITEM_THROW_GLIDE_TOSS:        i32 = 0x2C;
        pub const ITEM_THROW_GLIDING:           i32 = 0x2D;
        pub const ITEM_THROW_GLIDE_CHECK:       i32 = 0x2E;
        pub const ATTACK_DASH_CANCEL_DISABLE:   i32 = 0x2F;
        pub const ATTACK_DASH_SLIDEOFF:         i32 = 0x30;
        pub const PERFECT_WAVEDASH:             i32 = 0x31;
        pub const SHOULD_WAVELAND:              i32 = 0x32;

        // float consts
        pub const ECB_Y_OFFSET:                i32 = 0x0;
        pub const CURRENT_MOMENTUM:            i32 = 0x1;
        pub const CURRENT_MOMENTUM_SPECIALS:   i32 = 0x2;
        pub const JUMPSQUAT_VELOCITY:          i32 = 0x3;
        pub const GROUND_VELOCITY:             i32 = 0x4;
        pub const BASE_DASH_SPEED:             i32 = 0x5;
        pub const BASE_RUN_SPEED_MAX:          i32 = 0x6;
        pub const RAR_LENIENCY:                i32 = 0x7;
        pub const MOONWALK_JUMPSQUAT_VELOCITY: i32 = 0x8;
        pub const LEDGE_POSITION:              i32 = 0x9;
        pub const LEDGE_POS_X:                 i32 = 0x9;
        pub const LEDGE_POS_Y:                 i32 = 0xA;
        pub const LEDGE_POS_Z:                 i32 = 0xB;
        pub const SPEED_VECTOR:                i32 = 0xC;
        pub const SPEED_VEC_X:                 i32 = 0xC;
        pub const SPEED_VEC_Y:                 i32 = 0xD;
        pub const SPEED_VEC_Z:                 i32 = 0xE;
        pub const GET_DIST_TO_FLOOR:           i32 = 0xF;
        pub const Y_POS:                       i32 = 0x10;

        // int64 consts
        pub const ATTACK_JAB_CANCEL_MOTION: i32 = 0x0;
    }

    // fighter consts are allowed to be 0x1000 - 0x1FFF

    pub mod brave {
        // flag consts
        pub const CRITICAL_HIT: i32 = 0x1000;
    }

    pub mod captain {
        // int consts
        pub const SPECIAL_N_TURN_COUNT: i32 = 0x1000;
    }

    pub mod chrom {
        // flag consts
        pub use super::roy::SWORD_TRAIL_EFFECT;
        pub const SOARING_SLASH_HIT: i32 = 0x1001;
    }

    pub mod dolly {
        // int consts
        pub const EX_SEQUENCE_NUM: i32 = 0x1000;

        // flag consts
        pub use super::ryu::IS_HEAVY_ATTACK;
        pub const FIRE_KICK_USED:      i32 = 0x1001;
        pub const POWER_CHARGE_CANCEL: i32 = 0x1002;
        pub const SUPER_CANCEL:        i32 = 0x1003;
    }

    pub mod fox {
        // flag consts
        pub const ILLUSION_SHORTEN:   i32 = 0x1000;
        pub const ILLUSION_SHORTENED: i32 = 0x1001;
    }

    pub mod popo {
        // flag consts
        pub const GRAB_CANCEL_JUMP: i32 = 0x1000;
    }

    pub mod ken {
        pub use super::ryu::IS_HEAVY_ATTACK;
    }

    pub mod kirby {
        // flag consts
        pub const STAR_ROD_CONSUMED: i32 = 0x1000;
        pub const FINAL_CUTTER_HIT:  i32 = 0x1001;
    }

    pub mod link {
        // flag consts
        pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x1000;
    }

    pub mod lucas {
            // float consts
            pub use super::yoshi::JUMP_AERIAL_FRAME;
            pub use super::yoshi::JUMP_AERIAL_TIMER;

            // flag consts
            pub use super::yoshi::JUMP_AERIAL_STOP;
            pub use super::yoshi::JUMP_AERIAL_CANCEL;
    }

    pub mod ike {
        pub const SPECIAL_WALL_JUMP: i32 = 0x1000;
    }

    pub mod mario {
        // flag consts
        pub const AERIAL_CMD_RISING:         i32 = 0x1000;
        pub const AERIAL_CMD_RISEN:          i32 = 0x1001;
        pub const AERIAL_CMD_MOMENTUM_RESET: i32 = 0x1002;
        pub const NOKNOK_SHELL:              i32 = 0x1003;
    }

    pub mod master {
        // int consts
        pub const SPECIAL_AUTOCANCEL: i32 = 0x1000;
    }

    pub mod metaknight {
        // flag consts
        pub const SPECIAL_N_HIT:  i32 = 0x1000;
        pub const SPECIAL_HI_HIT: i32 = 0x1001;
        pub const SPECIAL_S_HIT:  i32 = 0x1002;
        pub const SPECIAL_LW_HIT: i32 = 0x1003;
    }

    pub mod palutena {
        // flag consts
        pub use super::ike::SPECIAL_WALL_JUMP;
    }

    pub mod peach {
        // float consts
        pub use super::yoshi::JUMP_AERIAL_FRAME;
        pub use super::yoshi::JUMP_AERIAL_TIMER;

        // flag consts
        pub use super::yoshi::JUMP_AERIAL_STOP;
        pub use super::yoshi::JUMP_AERIAL_CANCEL;
    }

    pub mod pichu {
        pub use super::pikachu::DISABLE_SPECIAL_JUMP_CANCEL;
    }

    pub mod pickel {
        // float consts
        pub const GLIDE_TIMER: i32 = 0x1000;

        // flag consts
        pub const IN_TUMBLE:    i32 = 0x1000;
        pub const TUMBLE_BEGIN: i32 = 0x1001;
    }

    pub mod pikachu {
        // flag consts
        pub const DISABLE_SPECIAL_JUMP_CANCEL: i32 = 0x1000;
    }

    pub mod roy {
        // flag consts
        pub const SWORD_TRAIL_EFFECT: i32 = 0x1000;
    }

    pub mod ryu {
        // flag consts
        pub const IS_HEAVY_ATTACK: i32 = 0x1000;
    }

    pub mod samus {
        // flag consts
        pub const SHINESPARK_READY: i32 = 0x1000;
        pub const SHINESPARK_USED:  i32 = 0x1001;
    }

    pub mod simon {
        // flag consts
        pub const AIR_CROSS: i32 = 0x1000;
    }

    pub mod snake {
        // int counts
        pub const GRENADE_COUNTER: i32 = 0x1000;
    }

    pub mod sonic {
        // flag consts
        pub const SPECIAL_S_JUMP_DISABLE: i32 = 0x1000;
        pub const PULSE_HITBOX_ENABLE:    i32 = 0x1001;
        pub const BLAST_ATTACK_ENABLE:    i32 = 0x1002;
        pub const AIR_ESCAPE_DISABLE:     i32 = 0x1003;

        // float consts
        pub const SPECIAL_S_DASH_FRAME: i32 = 0x1000;
    }

    pub mod toonlink {
        pub use super::link::SPIN_ATTACK_LAND_CANCEL;
    }

    pub mod yoshi {
        // float consts
        pub const JUMP_AERIAL_FRAME: i32 = 0x1000;
        pub const JUMP_AERIAL_TIMER: i32 = 0x1001;

        // flag consts
        pub use super::mario::AERIAL_CMD_RISING;
        pub use super::mario::AERIAL_CMD_RISEN;
        pub use super::mario::AERIAL_CMD_MOMENTUM_RESET;
        pub const JUMP_AERIAL_STOP:   i32 = 0x1010;
        pub const JUMP_AERIAL_CANCEL: i32 = 0x1011;
    }

    pub mod younglink {
        pub use super::link::SPIN_ATTACK_LAND_CANCEL;
    }

    pub mod zenigame {
        // float consts
        pub const WITHDRAW_FRAME: i32 = 0x1000;
    }

    // consts for indexing the global table
    pub mod globals {
        // 0x1
        pub const FIGHTER_KIND:          i32 = 0x2;
        pub const OBJECT_ID:             i32 = 0x3;
        // 0x4
        pub const MODULE_ACCESSOR:       i32 = 0x5;
        // 0x6
        pub const INIT_STATUS_FUNC:      i32 = 0x7;
        pub const IS_STOPPING:           i32 = 0x8;
        pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
        pub const PREV_STATUS_KIND:      i32 = 0xA;
        pub const STATUS_KIND:           i32 = 0xB;
        pub const STATUS_COUNT:          i32 = 0xC;
        // 0xD
        pub const CURRENT_FRAME:         i32 = 0xE;
        pub const CURRENT_FRAME2:        i32 = 0xF;
        // 0x10
        // 0x11 func ptr
        // 0x12
        pub const SUB_STATUS3:           i32 = 0x13;
        pub const SUB_STATUS2:           i32 = 0x14;
        pub const SUB_STATUS:            i32 = 0x15;
        pub const SITUATION_KIND:        i32 = 0x16;
        pub const PREV_SITUATION_KIND:   i32 = 0x17;
        pub const PREV_STATUS_FRAME:     i32 = 0x18;
        // 0x19
        pub const STICK_X:               i32 = 0x1A;
        pub const STICK_Y:               i32 = 0x1B;
        pub const FLICK_X:               i32 = 0x1C;
        pub const FLICK_Y:               i32 = 0x1D;
        pub const FLICK_Y_DIR:           i32 = 0x1E;
        pub const PAD_FLAG:              i32 = 0x1F;
        pub const CMD_CAT1:              i32 = 0x20;
        pub const CMD_CAT2:              i32 = 0x21;
        pub const CMD_CAT3:              i32 = 0x22;
        pub const CMD_CAT4:              i32 = 0x23;
        // 0x24
        // 0x25
        // 0x26
        // 0x27
        // 0x28 some substatus
        // 0x29 some substatus
        // 0x2A
        pub const CUSTOM_ROUTINE:        i32 = 0x2B;
        // 0x2C
        // 0x2D
        // 0x2E
        // 0x2F
        // 0x30
        // 0x31
        // 0x32 some substatus
    }
}
