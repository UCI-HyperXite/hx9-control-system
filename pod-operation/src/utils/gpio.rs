#[non_exhaustive]
pub struct GpioPins;

// Not discriminant but removes need to cast
impl GpioPins {
	const _RESERVED_I2C_SDA: u8 = 2;
	const _RESERVED_I2C_SCL: u8 = 3;
	pub const WHEEL_ENCODER_A: u8 = 23;
	pub const WHEEL_ENCODER_B: u8 = 24;
	pub const CONTACTOR_RELAY: u8 = 20;
	pub const SIGNAL_LIGHT_RELAY: u8 = 21;
	pub const PNEUMATICS_RELAY: u8 = 26;
}
