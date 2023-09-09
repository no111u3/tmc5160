//! Registers of the TMC5160 

pub trait Address {
    fn addr(self) -> u8;
}

/// Register addresses of the TMC5160
#[derive(Debug, Copy, Clone)]
#[allow(dead_code, non_camel_case_types)]
pub enum Registers {
    /* General configuration registers */
    /// Global configuration flags
    GCONF = 0x00,
    /// Global status flags
    GSTAT = 0x01,
    /// UART transmission counter
    IFCNT = 0x02,
    /// UART slave configuration
    SLAVECONF = 0x03,
    /// Read input / write output pins
    IO_INPUT_OUTPUT = 0x04,
    /// Position comparison register
    X_COMPARE = 0x05,
    /// OTP programming register
    OTP_PROG = 0x06,
    /// OTP read register
    OTP_READ = 0x07,
    /// Factory configuration (clock trim)
    FACTORY_CONF = 0x08,
    /// Short detector configuration
    SHORT_CONF = 0x09,
    /// Driver configuration
    DRV_CONF = 0x0A,
    /// Global scaling of motor current
    GLOBAL_SCALER = 0x0B,
    /// Offset calibration results
    OFFSET_READ = 0x0C,

    /* Velocity dependent driver feature control registers */
    /// Driver current control
    IHOLD_IRUN = 0x10,
    /// Delay before power down
    TPOWERDOWN = 0x11,
    /// Actual time between microsteps
    TSTEP = 0x12,
    /// Upper velocity for stealthChop voltage PWM mode
    TPWMTHRS = 0x13,
    /// Lower threshold velocity for switching on smart energy coolStep and stallGuard feature
    TCOOLTHRS = 0x14,
    /// Velocity threshold for switching into a different chopper mode and fullstepping
    THIGH = 0x15,

    /* Ramp generator motion control registers */
    /// Driving mode (Velocity, Positioning, Hold)
    RAMPMODE = 0x20,
    /// Actual motor position
    XACTUAL = 0x21,
    /// Actual  motor  velocity  from  ramp  generator
    VACTUAL = 0x22,
    /// Motor start velocity
    VSTART = 0x23,
    /// First acceleration between VSTART and V1
    A_1 = 0x24,
    /// First acceleration/deceleration phase target velocity
    V_1 = 0x25,
    /// Second acceleration between V1 and VMAX
    AMAX = 0x26,
    /// Target velocity in velocity mode
    VMAX = 0x27,
    /// Deceleration between VMAX and V1
    DMAX = 0x28,
    /// Deceleration between V1 and VSTOP
    /// Attention:  Do  not  set  0  in  positioning  mode, even if V1=0!
    D_1 = 0x2A,
    /// Motor stop velocity
    /// Attention: Set VSTOP > VSTART!
    /// Attention:  Do  not  set  0  in  positioning  mode, minimum 10 recommend!
    VSTOP = 0x2B,
    /// Waiting time after ramping down to zero velocity before next movement or direction inversion can start.
    TZEROWAIT = 0x2C,
    /// Target position for ramp mode
    XTARGET = 0x2D,

    /* Ramp generator driver feature control registers */
    /// Velocity threshold for enabling automatic commutation dcStep
    VDCMIN = 0x33,
    /// Switch mode configuration
    SW_MODE = 0x34,
    /// Ramp status and switch event status
    RAMP_STAT = 0x35,
    /// Ramp generator latch position upon programmable switch event
    XLATCH = 0x36,

    /* Encoder registers */
    /// Encoder configuration and use of N channel
    ENCMODE = 0x38,
    /// Actual encoder position
    X_ENC = 0x39,
    /// Accumulation constant
    ENC_CONST = 0x3A,
    /// Encoder status information
    ENC_STATUS = 0x3B,
    /// Encoder position latched on N event
    ENC_LATCH = 0x3C,
    /// Maximum number of steps deviation between encoder counter and XACTUAL for deviation warning
    ENC_DEVIATION = 0x3D,

    /* Motor driver registers */
    /// Microstep table entries. Add 0...7 for the next registers
    MSLUT_0_7 = 0x60,
    /// Look up table segmentation definition
    MSLUTSEL = 0x68,
    /// Absolute current at microstep table entries 0 and 256
    MSLUTSTART = 0x69,
    /// Actual position in the microstep table
    MSCNT = 0x6A,
    /// Actual microstep current
    MSCURACT = 0x6B,
    /// Chopper and driver configuration
    CHOPCONF = 0x6C,
    /// coolStep smart current control register and stallGuard2 configuration
    COOLCONF = 0x6D,
    /// dcStep automatic commutation configuration register
    DCCTRL = 0x6E,
    /// stallGuard2 to_val and driver error flags
    DRV_STATUS = 0x6F,
    /// stealthChop voltage PWM mode chopper configuration
    PWMCONF = 0x70,
    /// Results of stealthChop amplitude regulator.
    PWM_SCALE = 0x71,
    /// Automatically determined PWM config to_vals
    PWM_AUTO = 0x72,
    /// Number of input steps skipped due to dcStep. only with SD_MODE = 1
    LOST_STEPS = 0x73,
}

impl Address for Registers {
    fn addr(self) -> u8 {
        self as u8
    }
}


/// SPISTATUS
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct SpiStatus {
    pub status_stop_r: bool,
    pub status_stop_l: bool,
    pub position_reached: bool,
    pub velocity_reached: bool,
    pub standstill: bool,
    pub sg2: bool,
    pub driver_error: bool,
    pub reset_flag: bool,
}

impl SpiStatus {
    pub fn from(value: u8) -> Self {
        Self {
            status_stop_r: (value & 0b10000000) >> 7 == 1,
            status_stop_l: (value & 0b1000000) >> 6 == 1,
            position_reached: (value & 0b1000000) >> 5 == 1,
            velocity_reached: (value & 0b100000) >> 4 == 1,
            standstill: (value & 0b10000) >> 3 == 1,
            sg2: (value & 0b1000) >> 2 == 1,
            driver_error: (value & 0b10 >> 1) == 1,
            reset_flag: (value & 0b1) == 1,
        }
    }

    pub fn to_val(&self) -> u8 {
        let mut val = 0;
        val |= (self.status_stop_r as u8) << 7;
        val |= (self.status_stop_l as u8) << 7;
        val |= (self.position_reached as u8) << 7;
        val |= (self.velocity_reached as u8) << 7;
        val |= (self.standstill as u8) << 7;
        val |= (self.sg2 as u8) << 7;
        val |= (self.driver_error as u8) << 7;
        val |= (self.reset_flag as u8) << 7;
        val
    }
}


/// DRVSTATUS
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct DrvStatus {
    pub standstill: bool,
    pub olb: bool,
    pub ola: bool,
    pub s2gb: bool,
    pub s2ga: bool,
    pub sg2: bool,
    pub otpw: bool,
    pub ot: bool,
    pub stallguard: bool,
    pub cs_actual: u8,
    pub fsactive: bool,
    pub stealth: bool,
    pub s2vsb: bool,
    pub s2vsa: bool,
    pub sg_result: u16,
}

impl DrvStatus {
    pub fn from(value: u32) -> Self {
        Self {
            standstill: (value >> 31) & 0b1 == 1,
            olb: (value >> 30) & 0b1 == 1,
            ola: (value >> 29) & 0b1 == 1,
            s2gb: (value >> 28) & 0b1 == 1,
            s2ga: (value >> 27) & 0b1 == 1,
            sg2: (value >> 26) & 0b1 == 1,
            otpw: (value >> 25) & 0b1 == 1,
            ot: (value >> 24) & 0b1 == 1,
            stallguard: (value >> 23) & 0b1 == 1,
            cs_actual: ((value >> 16) & 0b11111) as u8,
            fsactive: (value >> 15) & 0b1 == 1,
            stealth: (value >> 14) & 0b1 == 1,
            s2vsb: (value >> 13) & 0b1 == 1,
            s2vsa: (value >> 12) & 0b1 == 1,
            sg_result: (value & 0b1111111111) as u16,
        }
    }
}

/// GCONF Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct GConf {
    pub recalibrate: bool,
    pub faststandstill: bool,
    pub en_pwm_mode: bool,
    pub multistep_filt: bool,
    pub shaft: bool,
    pub diag0_error: bool,
    pub diag0_otp: bool,
    pub diag0_stall: bool,
    pub diag1_stall: bool,
    pub diag1_index: bool,
    pub diag1_onstate: bool,
    pub diag1_steps_skipped: bool,
    pub diag0_int_pushpull: bool,
    pub diag1_poscomp_pushpull: bool,
    pub small_hysteresis: bool,
    pub stop_enable: bool,
    pub direct_mode: bool,
    test_mode: bool,
}

impl GConf {
    pub fn new() -> Self {
        Self {
            recalibrate: false,
            faststandstill: false,
            en_pwm_mode: true,
            multistep_filt: true,
            shaft: true,
            diag0_error: false,
            diag0_otp: false,
            diag0_stall: false,
            diag1_stall: false,
            diag1_index: false,
            diag1_onstate: false,
            diag1_steps_skipped: false,
            diag0_int_pushpull: false,
            diag1_poscomp_pushpull: false,
            small_hysteresis: false,
            stop_enable: false,
            direct_mode: false,
            test_mode: false,
        }
    }

    pub fn from(val: u32) -> Self {
        Self {
            recalibrate: (val & 0b1) == 1,
            faststandstill: ((val >> 1) & 0b1) == 1,
            en_pwm_mode: ((val >> 2) & 0b1) == 1,
            multistep_filt: ((val >> 3) & 0b1) == 1,
            shaft: ((val >> 4) & 0b1) == 1,
            diag0_error: ((val >> 5) & 0b1) == 1,
            diag0_otp: ((val >> 6) & 0b1) == 1,
            diag0_stall: ((val >> 7) & 0b1) == 1,
            diag1_stall: ((val >> 8) & 0b1) == 1,
            diag1_index: ((val >> 9) & 0b1) == 1,
            diag1_onstate: ((val >> 10) & 0b1) == 1,
            diag1_steps_skipped: ((val >> 11) & 0b1) == 1,
            diag0_int_pushpull: ((val >> 12) & 0b1) == 1,
            diag1_poscomp_pushpull: ((val >> 13) & 0b1) == 1,
            small_hysteresis: ((val >> 14) & 0b1) == 1,
            stop_enable: ((val >> 15) & 0b1) == 1,
            direct_mode: ((val >> 16) & 0b1) == 1,
            test_mode: ((val >> 17) & 0b1) == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.recalibrate as u32;
        val |= (self.faststandstill as u32) << 1;
        val |= (self.en_pwm_mode as u32) << 2;
        val |= (self.multistep_filt as u32) << 3;
        val |= (self.shaft as u32) << 4;
        val |= (self.diag0_error as u32) << 5;
        val |= (self.diag0_otp as u32) << 6;
        val |= (self.diag0_stall as u32) << 7;
        val |= (self.diag1_stall as u32) << 8;
        val |= (self.diag1_index as u32) << 9;
        val |= (self.diag1_onstate as u32) << 10;
        val |= (self.diag1_steps_skipped as u32) << 11;
        val |= (self.diag0_int_pushpull as u32) << 12;
        val |= (self.diag1_poscomp_pushpull as u32) << 13;
        val |= (self.small_hysteresis as u32) << 14;
        val |= (self.stop_enable as u32) << 15;
        val |= (self.direct_mode as u32) << 16;
        val |= (self.test_mode as u32) << 17;
        val
    }
}

/// GSTAT Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct GStat {
    pub reset: bool,
    pub drv_err: bool,
    pub uv_cp: bool,
}

impl GStat {
    pub fn from(val: u32) -> Self {
        Self {
            reset: (val & 0b100) >> 2 == 1,
            drv_err: (val & 0b10) >> 1 == 1,
            uv_cp: (val & 0b1) == 1,
        }
    }
}

// IFCNT Register is disabled in SPI mode

/// NODECONF Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct NodeConf {
    pub nodeaddr: u8,
    pub senddelay: u8,
}

impl NodeConf {
    pub fn from(val: u32) -> Self {
        Self {
            nodeaddr: (val & 0b11111111) as u8,
            senddelay: (val >> 8) as u8,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.nodeaddr as u32;
        val |= (self.senddelay as u32) << 8;
        val
    }
}


/// IOIN Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct IoIn {
    pub refl_step: bool,
    pub refr_dir: bool,
    pub encb_dcen_cfg4: bool,
    pub enca_dcen_cfg5: bool,
    pub drv_enn: bool,
    pub enc_n_dco_cfg6: bool,
    pub sd_mode: bool,
    pub swcomp_in: bool,
    pub version: u8,
}

impl IoIn {
    pub fn from(val: u32) -> Self {
        Self {
            refl_step: (val & 0b1) == 1,
            refr_dir: (val & 0b10) >> 1 == 1,
            encb_dcen_cfg4: (val & 0b100) >> 2 == 1,
            enca_dcen_cfg5: (val & 0b1000) >> 3 == 1,
            drv_enn: (val & 0b10000) >> 4 == 1,
            enc_n_dco_cfg6: (val & 0b100000) >> 5 == 1,
            sd_mode: (val & 0b1000000) >> 6 == 1,
            swcomp_in: (val & 0b10000000) >> 7 == 1,
            version: ((val >> 24) & 0b11111111) as u8,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.refl_step as u32;
        val |= (self.refr_dir as u32) << 1;
        val |= (self.encb_dcen_cfg4 as u32) << 2;
        val |= (self.enca_dcen_cfg5 as u32) << 3;
        val |= (self.drv_enn as u32) << 4;
        val |= (self.enc_n_dco_cfg6 as u32) << 5;
        val |= (self.sd_mode as u32) << 6;
        val |= (self.swcomp_in as u32) << 7;
        val |= (self.version as u32) << 24;
        val
    }
}

/// OTP_PROG Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct OtpProg {
    pub otpbit: u8,
    pub otpbyte: u8,
    pub otpmagic: u8,
}

impl OtpProg {
    pub fn from(val: u32) -> Self {
        Self {
            otpbit: (val & 0b11) as u8,
            otpbyte: ((val & 0b1100) >> 4) as u8,
            otpmagic: ((val >> 8) & 0b11111111) as u8,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.otpbit as u32;
        val |= (self.otpbyte as u32) << 4;
        val |= (self.otpmagic as u32) << 8;
        val
    }
}

/// OTPREAD
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct OtpRead {
    pub otp_tbl: bool,
    pub otp_bbm: bool,
    pub otp_s2_level: bool,
    pub otp_fclktrim: u8,
}

impl OtpRead {
    pub fn from(val: u32) -> Self {
        Self {
            otp_tbl: (val & 0b10000000) >> 7 == 1,
            otp_bbm: (val & 0b1000000) >> 6 == 1,
            otp_s2_level: (val & 0b100000) >> 5 == 1,
            otp_fclktrim: (val & 0b11111) as u8,
        }
    }
}

/// SHORT_CONF
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct ShortConf {
    pub s2vs_level: u8,
    pub s2g_level: u8,
    pub shortfilter: u8,
    pub shortdelay: bool,
}

impl ShortConf {
    pub fn from(val: u32) -> Self {
        Self {
            s2vs_level: (val & 0b111) as u8,
            s2g_level: ((val >> 8) & 0b1111) as u8,
            shortfilter: ((val >> 16) & 0b11) as u8,
            shortdelay: (val >> 18) & 0b1 == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.s2vs_level as u32;
        val |= (self.s2g_level as u32) << 8;
        val |= (self.shortfilter as u32) << 16;
        val |= (self.shortdelay as u32) << 18;
        val
    }
}

/// DrvConfRegister
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct DrvConf {
    /// "Break Before Make" duration specified in ns (0 to 24)
    pub bbm_time: u8,
    /// "Break Before Make" duration specified in clock cycles (0 to 15).
    pub bbm_clks: u8,
    /// over temperature selection
    pub ots_select: u8,
    /// MOSFET gate driver current (0 to 3)
    pub drv_strength: u8,
    /// filter time constant
    pub filt_isense: u8,
}

impl DrvConf {
    pub fn new() -> Self {
        DrvConf {
            bbm_time: 0,
            bbm_clks: 0,
            ots_select: 0,
            drv_strength: 0,
            filt_isense: 0,
        }
    }

    pub fn from(val: u32) -> Self {
        Self {
            bbm_time: (val & 0b11111) as u8,
            bbm_clks: ((val >> 8) & 0b1111) as u8,
            ots_select: ((val >> 16) & 0b11) as u8,
            drv_strength: ((val >> 18) & 0b11) as u8,
            filt_isense: ((val >> 20) & 0b11) as u8,
        }
    }
    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.bbm_time as u32;
        val |= (self.bbm_clks as u32) << 8;
        val |= (self.ots_select as u32) << 16;
        val |= (self.drv_strength as u32) << 18;
        val |= (self.filt_isense as u32) << 20;
        val
    }
}

/// OFFSET_READ
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct OffsetRead {
    pub phase_a: u8,
    pub phase_b: u8,
}

impl OffsetRead {
    pub fn from(val: u32) -> Self {
        Self {
            phase_a: ((val >> 8) & 0b11111111) as u8,
            phase_b: (val & 0b11111111) as u8,
        }
    }
}


/// IHOLD_IRUN Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct IHoldIRun {
    /// motor hold current
    pub i_hold: u8,
    /// motor run current
    pub i_run: u8,
    /// number of clock cycles after motion
    pub i_hold_delay: u8,
}

impl IHoldIRun {
    pub fn new() -> Self {
        Self {
            i_hold: 16,
            i_run: 31,
            i_hold_delay: 0,
        }
    }

    pub fn from(val: u32) -> Self {
        Self {
            i_hold: (val & 0b11111) as u8,
            i_run: ((val >> 8) & 0b11111) as u8,
            i_hold_delay: ((val >> 16) & 0b1111) as u8,
        }
    }
    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.i_hold as u32;
        val |= (self.i_run as u32) << 8;
        val |= (self.i_hold_delay as u32) << 16;
        val
    }
}

/// RAMPMODE Register
#[allow(dead_code)]
pub enum RampMode {
    PositioningMode = 0x00,
    /// using all A, D and V parameters
    VelocityModePos = 0x01,
    /// positive VMAX, using AMAX acceleration
    VelocityModeNeg = 0x02,
    /// negative VMAX, using AMAX acceleration
    /// velocity remains unchanged, unless stop event occurs
    HoldMode = 0x03,
}


/// SW_MODE Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct SwMode {
    pub stop_l_enable: bool,
    pub stop_r_enable: bool,
    pub pol_stop_l: bool,
    pub pol_stop_r: bool,
    pub swap_lr: bool,
    pub latch_l_active: bool,
    pub latch_l_inactive: bool,
    pub latch_r_active: bool,
    pub latch_r_inactive: bool,
    pub en_latch_encoder: bool,
    pub sg_stop: bool,
    pub en_softstop: bool,

}

impl SwMode {
    pub fn from(val: u32) -> Self {
        Self {
            stop_l_enable: val & 0b1 == 1,
            stop_r_enable: (val >> 1) & 0b1 == 1,
            pol_stop_l: (val >> 2) & 0b1 == 1,
            pol_stop_r: (val >> 3) & 0b1 == 1,
            swap_lr: (val >> 4) & 0b1 == 1,
            latch_l_active: (val >> 5) & 0b1 == 1,
            latch_l_inactive: (val >> 6) & 0b1 == 1,
            latch_r_active: (val >> 7) & 0b1 == 1,
            latch_r_inactive: (val >> 8) & 0b1 == 1,
            en_latch_encoder: (val >> 9) & 0b1 == 1,
            sg_stop: (val >> 10) & 0b1 == 1,
            en_softstop: (val >> 11) & 0b1 == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.stop_l_enable as u32;
        val |= (self.stop_r_enable as u32) << 1;
        val |= (self.pol_stop_l as u32) << 2;
        val |= (self.pol_stop_r as u32) << 3;
        val |= (self.swap_lr as u32) << 4;
        val |= (self.latch_l_active as u32) << 5;
        val |= (self.latch_l_inactive as u32) << 6;
        val |= (self.latch_r_active as u32) << 7;
        val |= (self.latch_r_inactive as u32) << 8;
        val |= (self.en_latch_encoder as u32) << 9;
        val |= (self.sg_stop as u32) << 10;
        val |= (self.en_softstop as u32) << 11;
        val
    }
}


/// RAMOSTAT Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct RampStat {
    pub status_stop_l: bool,
    pub status_stop_r: bool,
    pub status_latch_l: bool,
    pub status_latch_r: bool,
    pub event_stop_l: bool,
    pub event_stop_r: bool,
    pub event_stop_sg: bool,
    pub event_pos_reached: bool,
    pub velocity_reached: bool,
    pub position_reached: bool,
    pub vzero: bool,
    pub t_zerowait_active: bool,
    pub second_move: bool,
    pub status_sg: bool,
}

impl RampStat {
    pub fn from(val: u32) -> Self {
        Self {
            status_stop_l: val & 0b1 == 1,
            status_stop_r: (val >> 1) & 0b1 == 1,
            status_latch_l: (val >> 2) & 0b1 == 1,
            status_latch_r: (val >> 3) & 0b1 == 1,
            event_stop_l: (val >> 4) & 0b1 == 1,
            event_stop_r: (val >> 5) & 0b1 == 1,
            event_stop_sg: (val >> 6) & 0b1 == 1,
            event_pos_reached: (val >> 7) & 0b1 == 1,
            velocity_reached: (val >> 8) & 0b1 == 1,
            position_reached: (val >> 9) & 0b1 == 1,
            vzero: (val >> 10) & 0b1 == 1,
            t_zerowait_active: (val >> 11) & 0b1 == 1,
            second_move: (val >> 12) & 0b1 == 1,
            status_sg: (val >> 13) & 0b1 == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.status_stop_l as u32;
        val |= (self.status_stop_r as u32) << 1;
        val |= (self.status_latch_l as u32) << 2;
        val |= (self.status_latch_r as u32) << 3;
        val |= (self.event_stop_l as u32) << 4;
        val |= (self.event_stop_r as u32) << 5;
        val |= (self.event_stop_sg as u32) << 6;
        val |= (self.event_pos_reached as u32) << 7;
        val |= (self.velocity_reached as u32) << 8;
        val |= (self.position_reached as u32) << 9;
        val |= (self.vzero as u32) << 10;
        val |= (self.t_zerowait_active as u32) << 11;
        val |= (self.second_move as u32) << 12;
        val |= (self.status_sg as u32) << 13;
        val
    }
}


/// ENCMODE Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct EncMode {
    pub pol_a: bool,
    pub pol_b: bool,
    pub pol_n: bool,
    pub ignore_ab: bool,
    pub clr_cont: bool,
    pub clr_once: bool,
    pub pos_edge: bool,
    pub neg_edge: bool,
    pub clr_enc_x: bool,
    pub latch_x_act: bool,
    pub enc_sel_decimal: bool,
}

impl EncMode {
    pub fn from(val: u32) -> Self {
        Self {
            pol_a: (val & 0b1) == 1,
            pol_b: ((val >> 1) & 0b1) == 1,
            pol_n: ((val >> 2) & 0b1) == 1,
            ignore_ab: ((val >> 3) & 0b1) == 1,
            clr_cont: ((val >> 4) & 0b1) == 1,
            clr_once: ((val >> 5) & 0b1) == 1,
            pos_edge: ((val >> 6) & 0b1) == 1,
            neg_edge: ((val >> 7) & 0b1) == 1,
            clr_enc_x: ((val >> 8) & 0b1) == 1,
            latch_x_act: ((val >> 9) & 0b1) == 1,
            enc_sel_decimal: ((val >> 10) & 0b1) == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.pol_a as u32;
        val |= (self.pol_b as u32) << 1;
        val |= (self.pol_n as u32) << 2;
        val |= (self.ignore_ab as u32) << 3;
        val |= (self.clr_cont as u32) << 4;
        val |= (self.clr_once as u32) << 5;
        val |= (self.pos_edge as u32) << 6;
        val |= (self.neg_edge as u32) << 7;
        val |= (self.clr_enc_x as u32) << 8;
        val |= (self.latch_x_act as u32) << 9;
        val |= (self.enc_sel_decimal as u32) << 10;
        val
    }
}


/// MSLUTSEL Register
#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
pub struct MsLutSel {
    pub w0: u8,
    pub w1: u8,
    pub w2: u8,
    pub w3: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

impl MsLutSel {
    pub fn from(val: u32) -> Self {
        Self {
            w0: (val & 0b11) as u8,
            w1: ((val >> 2) & 0b11) as u8,
            w2: ((val >> 4) & 0b11) as u8,
            w3: ((val >> 6) & 0b11) as u8,
            x1: ((val >> 8) & 0b11) as u8,
            x2: ((val >> 16) & 0b11) as u8,
            x3: ((val >> 24) & 0b11) as u8,
        }
    }
    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.w0 as u32;
        val |= (self.w1 as u32) << 2;
        val |= (self.w2 as u32) << 4;
        val |= (self.w3 as u32) << 6;
        val |= (self.x1 as u32) << 8;
        val |= (self.x2 as u32) << 16;
        val |= (self.x3 as u32) << 24;
        val
    }
}

/// CHOPFCONF Register
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ChopConf {
    pub toff: u8,
    pub hstr: u8,
    pub hend: u8,
    pub fd3: bool,
    pub disfdcc: bool,
    pub chm: bool,
    pub tbl: u8,
    pub vhighfs: bool,
    pub vhighchm: bool,
    pub tpfd: u8,
    pub mres: u8,
    pub intpol: bool,
    pub dedge: bool,
    pub diss2g: bool,
    pub diss2vs: bool,
}

impl Default for ChopConf {
    fn default() -> Self {
        Self {
            toff: ((0x10410150_u32) & 0b1111) as u8,
            hstr: ((0x10410150_u32 >> 4) & 0b111) as u8,
            hend: ((0x10410150_u32 >> 7) & 0b1111) as u8,
            fd3: ((0x10410150_u32 >> 11) & 0b1) == 1,
            disfdcc: ((0x10410150_u32 >> 12) & 0b1) == 1,
            chm: ((0x10410150_u32 >> 14) & 0b1) == 1,
            tbl: ((0x10410150_u32 >> 15) & 0b11) as u8,
            vhighfs: ((0x10410150_u32 >> 18) & 0b1) == 1,
            vhighchm: ((0x10410150_u32 >> 19) & 0b1) == 1,
            tpfd: ((0x10410150_u32 >> 20) & 0b1111) as u8,
            mres: ((0x10410150_u32 >> 24) & 0b1111) as u8,
            intpol: ((0x10410150_u32 >> 28) & 0b1) == 1,
            dedge: ((0x10410150_u32 >> 29) & 0b1) == 1,
            diss2g: ((0x10410150_u32 >> 30) & 0b1) == 1,
            diss2vs: ((0x10410150_u32 >> 31) & 0b1) == 1,
        }
    }
}

impl ChopConf {
    pub fn from(val: u32) -> Self {
        Self {
            toff: ((val) & 0b1111) as u8,
            hstr: ((val >> 4) & 0b111) as u8,
            hend: ((val >> 7) & 0b1111) as u8,
            fd3: ((val >> 11) & 0b1) == 1,
            disfdcc: ((val >> 12) & 0b1) == 1,
            chm: ((val >> 14) & 0b1) == 1,
            tbl: ((val >> 15) & 0b11) as u8,
            vhighfs: ((val >> 18) & 0b1) == 1,
            vhighchm: ((val >> 19) & 0b1) == 1,
            tpfd: ((val >> 20) & 0b1111) as u8,
            mres: ((val >> 24) & 0b1111) as u8,
            intpol: ((val >> 28) & 0b1) == 1,
            dedge: ((val >> 29) & 0b1) == 1,
            diss2g: ((val >> 30) & 0b1) == 1,
            diss2vs: ((val >> 31) & 0b1) == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.toff as u32;
        val |= (self.hstr as u32) << 4;
        val |= (self.hend as u32) << 7;
        val |= (self.fd3 as u32) << 11;
        val |= (self.disfdcc as u32) << 13;
        val |= (self.chm as u32) << 14;
        val |= (self.tbl as u32) << 15;
        val |= (self.vhighfs as u32) << 18;
        val |= (self.vhighchm as u32) << 19;
        val |= (self.tpfd as u32) << 20;
        val |= (self.mres as u32) << 24;
        val |= (self.intpol as u32) << 28;
        val |= (self.dedge as u32) << 29;
        val |= (self.diss2g as u32) << 30;
        val |= (self.diss2vs as u32) << 31;
        val
    }
}

/// COOLCONF Register
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CoolConf {
    pub semin: u8,
    pub seup: u8,
    pub semax: u8,
    pub sedn: u8,
    pub seimin: bool,
    pub sgt: u8,
    pub sfilt: bool,
}

impl Default for CoolConf {
    fn default() -> Self {
        Self {
            semin: 0,
            seup: 0,
            semax: 0,
            sedn: 0,
            seimin: false,
            sgt: 0,
            sfilt: false,
        }
    }
}

impl CoolConf {
    pub fn from(val: u32) -> Self {
        Self {
            semin: (val & 0b1111) as u8,
            seup: ((val >> 5) & 0b11) as u8,
            semax: ((val >> 8) & 0b1111) as u8,
            sedn: ((val >> 13) & 0b11) as u8,
            seimin: ((val >> 15) & 0b1) == 1,
            sgt: ((val >> 16) & 0b1111111) as u8,
            sfilt: ((val >> 24) & 0b1) == 1,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.semin as u32;
        val |= (self.seup as u32) << 5;
        val |= (self.semax as u32) << 8;
        val |= (self.sedn as u32) << 13;
        val |= (self.semin as u32) << 15;
        val |= (self.sgt as u32) << 16;
        val |= (self.sfilt as u32) << 24;
        val
    }
}

/// PWMCONF Register
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct PwmConf {
    pub pwm_ofs: u8,
    pub pwm_grad: u8,
    pub pwm_freq: u8,
    pub pwm_autoscale: bool,
    pub pwm_autograd: bool,
    pub free_wheel: u8,
    pub pwm_reg: u8,
    pub pwm_lim: u8,
}

impl Default for PwmConf {
    fn default() -> Self {
        Self {
            pwm_ofs: ((0xC40C001E_u32) & 0xff) as u8,
            pwm_grad: ((0xC40C001E_u32 >> 8) & 0xff) as u8,
            pwm_freq: ((0xC40C001E_u32 >> 16) & 0b11) as u8,
            pwm_autoscale: ((0xC40C001E_u32 >> 18) & 0b1) == 1,
            pwm_autograd: ((0xC40C001E_u32 >> 19) & 0b1) == 1,
            free_wheel: ((0xC40C001E_u32 >> 20) & 0b11) as u8,
            pwm_reg: ((0xC40C001E_u32 >> 24) & 0b1111) as u8,
            pwm_lim: ((0xC40C001E_u32 >> 28) & 0b1111) as u8,
        }
    }
}

impl PwmConf {
    pub fn from(val: u32) -> Self {
        Self {
            pwm_ofs: ((val) & 0xff) as u8,
            pwm_grad: ((val >> 8) & 0xff) as u8,
            pwm_freq: ((val >> 16) & 0b11) as u8,
            pwm_autoscale: ((val >> 18) & 0b1) == 1,
            pwm_autograd: ((val >> 19) & 0b1) == 1,
            free_wheel: ((val >> 20) & 0b11) as u8,
            pwm_reg: ((val >> 24) & 0b1111) as u8,
            pwm_lim: ((val >> 28) & 0b1111) as u8,
        }
    }

    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.pwm_ofs as u32;
        val |= (self.pwm_grad as u32) << 8;
        val |= (self.pwm_freq as u32) << 16;
        val |= (self.pwm_autoscale as u32) << 18;
        val |= (self.pwm_autograd as u32) << 19;
        val |= (self.free_wheel as u32) << 20;
        val |= (self.pwm_reg as u32) << 24;
        val |= (self.pwm_lim as u32) << 28;
        val
    }
}