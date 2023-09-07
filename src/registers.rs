//! Registers of the TMC5160 


pub trait Address {
    fn addr(self) -> u8;
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Registers {
    GCONF = 0x00,
    GSTAT = 0x01,
    IFCNT = 0x02,
    SLAVECONF = 0x03,
    INP_OUT = 0x04,
    X_COMPARE = 0x05,
    OTP_PROG = 0x06,
    OTP_READ = 0x07,
    FACTORY_CONF = 0x08,
    SHORT_CONF = 0x09,
    DRV_CONF = 0x0A,
    GLOBAL_SCALER = 0x0B,
    OFFSET_READ = 0x0C,
    IHOLD_IRUN = 0x10,
    TPOWERDOWN = 0x11,
    TSTEP = 0x12,
    TPWMTHRS = 0x13,
    TCOOLTHRS = 0x14,
    THIGH = 0x15,

    RAMPMODE = 0x20,
    XACTUAL = 0x21,
    VACTUAL = 0x22,
    VSTART = 0x23,
    A1 = 0x24,
    V1 = 0x25,
    AMAX = 0x26,
    VMAX = 0x27,
    DMAX = 0x28,
    D1 = 0x2A,
    VSTOP = 0x2B,
    TZEROWAIT = 0x2C,
    XTARGET = 0x2D,

    VDCMIN = 0x33,
    SWMODE = 0x34,
    RAMPSTAT = 0x35,
    XLATCH = 0x36,
    ENCMODE = 0x38,
    XENC = 0x39,
    ENC_CONST = 0x3A,
    ENC_STATUS = 0x3B,
    ENC_LATCH = 0x3C,
    ENC_DEVIATION = 0x3D,

    MSLUT0 = 0x60,
    MSLUT1 = 0x61,
    MSLUT2 = 0x62,
    MSLUT3 = 0x63,
    MSLUT4 = 0x64,
    MSLUT5 = 0x65,
    MSLUT6 = 0x66,
    MSLUT7 = 0x67,
    MSLUTSEL = 0x68,
    MSLUTSTART = 0x69,
    MSCNT = 0x6A,
    MSCURACT = 0x6B,
    CHOPCONF = 0x6C,
    COOLCONF = 0x6D,
    DCCTRL = 0x6E,
    DRVSTATUS = 0x6F,
    PWMCONF = 0x70,
    PWMSCALE = 0x71,
    PWM_AUTO = 0x72,
    LOST_STEPS = 0x73,
}

impl Address for Registers {
    fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
enum RampMode {
    PositioningMode = 0x00,
    /// using all A, D and V parameters
    VelocityModePos = 0x01,
    /// positive VMAX, using AMAX acceleration
    VelocityModeNeg = 0x02,
    /// negative VMAX, using AMAX acceleration
    HoldMode = 0x03, // velocity remains unchanged, unless stop event occurs
}

#[allow(dead_code, non_camel_case_types)]
enum Tmc5160Reg {
    /* Register addresses */
    /* General configuration registers */
    GCONF = 0x00,
    // Global configuration flags
    GSTAT = 0x01,
    // Global status flags
    IFCNT = 0x02,
    // UART transmission counter
    SLAVECONF = 0x03,
    // UART slave configuration
    IO_INPUT_OUTPUT = 0x04,
    // Read input / write output pins
    X_COMPARE = 0x05,
    // Position comparison register
    OTP_PROG = 0x06,
    // OTP programming register
    OTP_READ = 0x07,
    // OTP read register
    FACTORY_CONF = 0x08,
    // Factory configuration (clock trim)
    SHORT_CONF = 0x09,
    // Short detector configuration
    DRV_CONF = 0x0A,
    // Driver configuration
    GLOBAL_SCALER = 0x0B,
    // Global scaling of motor current
    OFFSET_READ = 0x0C, // Offset calibration results

    /* Velocity dependent driver feature control registers */
    IHOLD_IRUN = 0x10,
    // Driver current control
    TPOWERDOWN = 0x11,
    // Delay before power down
    TSTEP = 0x12,
    // Actual time between microsteps
    TPWMTHRS = 0x13,
    // Upper velocity for stealthChop voltage PWM mode
    TCOOLTHRS = 0x14,
    // Lower threshold velocity for switching on smart energy coolStep and stallGuard feature
    THIGH = 0x15, // Velocity threshold for switching into a different chopper mode and fullstepping

    /* Ramp generator motion control registers */
    RAMPMODE = 0x20,
    // Driving mode (Velocity, Positioning, Hold)
    XACTUAL = 0x21,
    // Actual motor position
    VACTUAL = 0x22,
    // Actual  motor  velocity  from  ramp  generator
    VSTART = 0x23,
    // Motor start velocity
    A_1 = 0x24,
    // First acceleration between VSTART and V1
    V_1 = 0x25,
    // First acceleration/deceleration phase target velocity
    AMAX = 0x26,
    // Second acceleration between V1 and VMAX
    VMAX = 0x27,
    // Target velocity in velocity mode
    DMAX = 0x28,
    // Deceleration between VMAX and V1
    D_1 = 0x2A,
    // Deceleration between V1 and VSTOP
    //Attention:  Do  not  set  0  in  positioning  mode, even if V1=0!
    VSTOP = 0x2B,
    // Motor stop velocity
    //Attention: Set VSTOP > VSTART!
    //Attention:  Do  not  set  0  in  positioning  mode, minimum 10 recommend!
    TZEROWAIT = 0x2C,
    // Waiting time after ramping down to zero velocity before next movement or direction inversion can start.
    XTARGET = 0x2D, // Target position for ramp mode

    /* Ramp generator driver feature control registers */
    VDCMIN = 0x33,
    // Velocity threshold for enabling automatic commutation dcStep
    SW_MODE = 0x34,
    // Switch mode configuration
    RAMP_STAT = 0x35,
    // Ramp status and switch event status
    XLATCH = 0x36, // Ramp generator latch position upon programmable switch event

    /* Encoder registers */
    ENCMODE = 0x38,
    // Encoder configuration and use of N channel
    X_ENC = 0x39,
    // Actual encoder position
    ENC_CONST = 0x3A,
    // Accumulation constant
    ENC_STATUS = 0x3B,
    // Encoder status information
    ENC_LATCH = 0x3C,
    // Encoder position latched on N event
    ENC_DEVIATION = 0x3D, // Maximum number of steps deviation between encoder counter and XACTUAL for deviation warning

    /* Motor driver registers */
    MSLUT_0_7 = 0x60,
    // Microstep table entries. Add 0...7 for the next registers
    MSLUTSEL = 0x68,
    // Look up table segmentation definition
    MSLUTSTART = 0x69,
    // Absolute current at microstep table entries 0 and 256
    MSCNT = 0x6A,
    // Actual position in the microstep table
    MSCURACT = 0x6B,
    // Actual microstep current
    CHOPCONF = 0x6C,
    // Chopper and driver configuration
    COOLCONF = 0x6D,
    // coolStep smart current control register and stallGuard2 configuration
    DCCTRL = 0x6E,
    // dcStep automatic commutation configuration register
    DRV_STATUS = 0x6F,
    // stallGuard2 to_val and driver error flags
    PWMCONF = 0x70,
    // stealthChop voltage PWM mode chopper configuration
    PWM_SCALE = 0x71,
    // Results of stealthChop amplitude regulator.
    PWM_AUTO = 0x72,
    // Automatically determined PWM config to_vals
    LOST_STEPS = 0x73, // Number of input steps skipped due to dcStep. only with SD_MODE = 1
}

pub struct GConfRegister {
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

impl GConfRegister {
    pub fn new() -> Self {
        GConfRegister {
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

pub struct ChopConfRegister {
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

impl ChopConfRegister {
    pub fn new() -> Self {
        ChopConfRegister {
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

pub struct CoolConfRegister {
    pub sfilt: bool,
    pub sgt: u8,
    pub seimin: bool,
    pub sedn: u8,
    pub semax: u8,
    pub seup: u8,
    pub semin: u8,
}

impl CoolConfRegister {
    pub fn new() -> Self {
        CoolConfRegister {
            sfilt: false,
            sgt: 1,
            seimin: false,
            sedn: 0,
            semax: 0,
            seup: 0,
            semin: 0,
        }
    }
    pub fn to_val(&self) -> u32 {
        let mut val = 0;
        val |= self.semin as u32;
        val |= (self.seup as u32) << 8;
        val |= (self.semax as u32) << 13;
        val |= (self.sedn as u32) << 14;
        val |= (self.semin as u32) << 15;
        val |= (self.sgt as u32) << 16;
        val |= (self.sfilt as u32) << 24;
        val
    }
}

pub struct PwmConfRegister {
    pub pwm_ofs: u8,
    pub pwm_grad: u8,
    pub pwm_freq: u8,
    pub pwm_autoscale: bool,
    pub pwm_autograd: bool,
    pub free_wheel: u8,
    pub pwm_reg: u8,
    pub pwm_lim: u8,
}

impl PwmConfRegister {
    pub fn new() -> Self {
        PwmConfRegister {
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

pub struct DrvConfRegister {
    pub bbm_time: u8,
    // "Break Before Make" duration specified in ns (0 to 24)
    pub bbm_clks: u8,
    // "Break Before Make" duration specified in clock cycles (0 to 15).
    pub ots_select: u8,
    // over temperature selection
    pub drv_strength: u8,
    // MOSFET gate driver current (0 to 3)
    pub filt_isense: u8, // filter time constant
}

impl DrvConfRegister {
    pub fn new() -> Self {
        DrvConfRegister {
            bbm_time: 0,
            bbm_clks: 0,
            ots_select: 0,
            drv_strength: 0,
            filt_isense: 0,
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

pub struct IHoldIRunRegister {
    pub i_hold: u8,
    // motor hold current
    pub i_run: u8,
    // motor run current
    pub i_hold_delay: u8, // number of clock cycles after motion
}

impl IHoldIRunRegister {
    pub fn new() -> Self {
        IHoldIRunRegister {
            i_hold: 16,
            i_run: 31,
            i_hold_delay: 0,
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