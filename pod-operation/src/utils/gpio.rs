#[derive(num_enum::IntoPrimitive)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GpioPins {
	_RESERVED_I2C_SDA = 2,
	_RESERVED_I2C_SCL = 3,
	WHEEL_ENCODER_A = 14,
	WHEEL_ENCODER_B = 15,
	CONTACTOR_RELAY = 20,
	SIGNAL_LIGHT_RELAY = 21,
	PNEUMATICS_RELAY = 16,
	PICO_RELAY = 26,
}
