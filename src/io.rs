use rppal::{i2c::I2c, pwm::Channel};
use anyhow::Result;

const PCA9685_ADDR: u16 = 0x40; // Default I2C address
const SERVO_CHANNEL: u8 = 0; // Servo connected to channel 0
const PWM_FREQ_HZ: f64 = 50.0;
const PWM_RESOLUTION: u16 = 4096;

/// Converts the angle we want to turn to the pulse width.
pub fn angle_to_pulse_width(angle: f64) -> u16 {
    let min = 150; // a bit before 1 ms
    let max = 600; // a bit after 2 ms (to be safe)
    min + ((max - min) as f64 * angle / 180.0) as u16
}

pub fn write_to_pca(
    // Channel is which PWM output channel to control
    channel: u8,
    // The point in the 12-bit PWM cycle when the signal should turn off.
    on_value: u16) -> Result<()>
{
    let mut i2c = I2c::new()?;
    // Tells the peripheral to talk to the device at the specified address.
    i2c.set_slave_address(PCA9685_ADDR)?;

    // Set PWM frequency to 50 Hz (only needs to be done once ideally)
    let prescale_val = ((25_000_000.0 / (4096.0 * PWM_FREQ_HZ)) - 1.0).round() as u8;
    i2c.smbus_write_byte(0x00, 0x10)?; // Sleep mode
    i2c.smbus_write_byte(0xFE, prescale_val)?; // Prescale register
    i2c.smbus_write_byte(0x00, 0x20)?; // Wake up and auto-increment

    // Calculate registers
    let on_l = 0x06 + 4 * channel;
    let off_val = on_value;
    let on_val = 0;

    i2c.smbus_write_byte(on_l, (on_val & 0xFF) as u8)?;
    i2c.smbus_write_byte(on_l + 1, (on_val >> 8) as u8)?;
    i2c.smbus_write_byte(on_l + 2, (off_val & 0xFF) as u8)?;
    i2c.smbus_write_byte(on_l + 3, (off_val >> 8) as u8)?;

    Ok(())
}
