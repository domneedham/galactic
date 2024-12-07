use embassy_rp::{
    gpio::Input,
    peripherals::{
        PIN_0, PIN_1, PIN_13, PIN_14, PIN_15, PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21,
        PIN_26, PIN_27, PIN_28, PIN_3, PIN_6, PIN_7, PIN_8,
    },
};

pub struct UnicornDisplayPins {
    pub column_clock: PIN_13,
    pub column_data: PIN_14,
    pub column_latch: PIN_15,
    pub column_blank: PIN_16,
    pub row_bit_0: PIN_17,
    pub row_bit_1: PIN_18,
    pub row_bit_2: PIN_19,
    pub row_bit_3: PIN_20,
}

pub struct UnicornSensorPins {
    pub light_sensor: PIN_28,
}

pub struct UnicornButtonPins<'d> {
    pub switch_a: Input<'d>,
    pub switch_b: Input<'d>,
    pub switch_c: Input<'d>,
    pub switch_d: Input<'d>,
    pub brightness_up: Input<'d>,
    pub brightness_down: Input<'d>,
    pub volume_up: Input<'d>,
    pub volume_down: Input<'d>,
    pub sleep: Input<'d>,
}
