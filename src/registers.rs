//! Registers of the TMC5160 
use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;

/// Implementation to convert register enum to u8 address
pub trait Address {
    /// convert register enum to u8 address
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
    IOIN = 0x04,
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
    GLOBALSCALER = 0x0B,
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
    A1 = 0x24,
    /// First acceleration/deceleration phase target velocity
    V1 = 0x25,
    /// Second acceleration between V1 and VMAX
    AMAX = 0x26,
    /// Target velocity in velocity mode
    VMAX = 0x27,
    /// Deceleration between VMAX and V1
    DMAX = 0x28,
    /// Deceleration between V1 and VSTOP
    /// Attention:  Do  not  set  0  in  positioning  mode, even if V1=0!
    D1 = 0x2A,
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
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 8)]
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


/// DRVSTATUS
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct DrvStatus {
    pub standstill: bool,
    pub olb: bool,
    pub ola: bool,
    pub s2gb: bool,
    pub s2ga: bool,
    pub otpw: bool,
    pub ot: bool,
    pub stallguard: bool,
    #[skip] _b: B3,
    pub cs_actual: B5,
    pub fsactive: bool,
    pub stealth: bool,
    pub s2vsb: bool,
    pub s2vsa: bool,
    #[skip] _a: B2,
    pub sg_result: B10,
}

/// GCONF Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
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
    #[skip] test_mode: bool,
    #[skip] _fill: B14,
}

/// GSTAT Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct GStat {
    pub reset: bool,
    pub drv_err: bool,
    pub uv_cp: bool,
    #[skip] _fill: B29,
}

// IFCNT Register is disabled in SPI mode

/// NODECONF Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct NodeConf {
    pub nodeaddr: u8,
    pub senddelay: B4,
    #[skip] _fill: B20,
}


/// IOIN Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
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
    #[skip] _fill: B16,
}


/// OTP_PROG Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct OtpProg {
    pub otpbit: B3,
    pub otpbyte: B2,
    #[skip] __: B3,
    pub otpmagic: u8,
    #[skip] _fill: B16,
}

/// OTPREAD
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct OtpRead {
    pub otp_fclktrim: B5,
    pub otp_s2_level: bool,
    pub otp_bbm: bool,
    pub otp_tbl: bool,
    #[skip] _fill: B24,
}

/// SHORT_CONF
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct ShortConf {
    pub s2vs_level: B4,
    #[skip] _a: B4,
    pub s2g_level: B4,
    #[skip] _b: B4,
    pub shortfilter: B2,
    pub shortdelay: bool,
    #[skip] _fill: B13,
}

/// DrvConfRegister
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct DrvConf {
    /// "Break Before Make" duration specified in ns (0 to 24)
    pub bbm_time: B4,
    #[skip] _a: B4,
    /// "Break Before Make" duration specified in clock cycles (0 to 15).
    pub bbm_clks: B4,
    #[skip] _b: B4,
    /// over temperature selection
    pub ots_select: B2,
    /// MOSFET gate driver current (0 to 3)
    pub drv_strength: B2,
    /// filter time constant
    pub filt_isense: B2,
    #[skip] _fill: B10,
}

/// OFFSET_READ
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield]
pub struct OffsetRead {
    pub phase_a: u8,
    pub phase_b: u8,
}


/// IHOLD_IRUN Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct IHoldIRun {
    /// motor hold current
    pub i_hold: B5,
    #[skip] _a: B3,
    /// motor run current
    pub i_run: B5,
    #[skip] _b: B3,
    /// number of clock cycles after motion
    pub i_hold_delay: B4,
    #[skip] _c: B4,
    #[skip] _d: B8,
}

/// RAMPMODE Register
#[allow(dead_code)]
pub enum RampMode {
    /// using all A, D and V parameters
    PositioningMode = 0x00,
    /// positive VMAX, using AMAX acceleration
    VelocityModePos = 0x01,
    /// negative VMAX, using AMAX acceleration
    VelocityModeNeg = 0x02,
    /// velocity remains unchanged, unless stop event occurs
    HoldMode = 0x03,
}


/// SW_MODE Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
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
    #[skip] _fill: B20,
}


/// RAMOSTAT Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
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
    #[skip] _fill: B18,
}


/// ENCMODE Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
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
    #[skip] _fill: B21,
}


/// MSLUTSEL Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct MsLutSel {
    pub w0: B2,
    pub w1: B2,
    pub w2: B2,
    pub w3: B2,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

/// CHOPFCONF Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct ChopConf {
    pub toff: B4,
    pub hstr: B3,
    pub hend: B4,
    pub fd3: bool,
    pub disfdcc: bool,
    #[skip] _a: B1,
    pub chm: bool,
    pub tbl: B2,
    #[skip] _b: B1,
    pub vhighfs: bool,
    pub vhighchm: bool,
    pub tpfd: B4,
    pub mres: B4,
    pub intpol: bool,
    pub dedge: bool,
    pub diss2g: bool,
    pub diss2vs: bool,
}

impl Default for ChopConf {
    fn default() -> Self {
        Self::from_bytes(0x10410150_u32.to_be_bytes())
    }
}


/// COOLCONF Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct CoolConf {
    pub semin: B4,
    #[skip] _a: B1,
    pub seup: B2,
    #[skip] _b: B1,
    pub semax: B4,
    #[skip] _c: B1,
    pub sedn: B2,
    pub seimin: bool,
    pub sgt: B6,
    #[skip] _d: B1,
    pub sfilt: bool,
    #[skip] _e: B8,
}

/// PWMCONF Register
#[derive(Clone, Copy)]
#[allow(dead_code)]
#[bitfield(bits = 32)]
pub struct PwmConf {
    pub pwm_ofs: u8,
    pub pwm_grad: u8,
    pub pwm_freq: B2,
    pub pwm_autoscale: bool,
    pub pwm_autograd: bool,
    pub free_wheel: B2,
    #[skip] __: B2,
    pub pwm_reg: B4,
    pub pwm_lim: B4,
}

impl Default for PwmConf {
    fn default() -> Self {
        Self::from_bytes(0xC40C001E_u32.to_be_bytes())
    }
}