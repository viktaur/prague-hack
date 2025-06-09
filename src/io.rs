use rppal::{i2c::I2c, pwm::Channel};
use anyhow::Result;

use crate::{MAX_ANGLE, MIN_ANGLE};

// Default I2C address. It's just a way for the master to address the slave device.
const PCA9685_ADDR: u16 = 0x40;
const SERVO_CHANNEL: u8 = 0; // Servo connected to channel 0. If we had another one is would be 1, and 2, and so on.
// This is the frequency that the servo expects.
const PWM_FREQ_HZ: f64 = 50.0;
// This is because we have 12 bits to represent each of the PWM steps (2^12)
const PWM_RESOLUTION: u16 = 4096;

/// Converts the positional angle we want to the corresponding steps.
pub fn angle_to_pulse_width(angle: f32) -> u16 {
    let angle = angle.clamp(MIN_ANGLE, MAX_ANGLE);
    let min_pulse = 102.4; // approx 0.5 ms (before, 150)
    let max_pulse = 512.0; // approx 2.5 ms (before, 450)
    let center = (min_pulse + max_pulse) / 2.0;
    let range = (max_pulse - min_pulse) / 2.0;
    let pulse_width = (center + angle / ((MAX_ANGLE - MIN_ANGLE) / 2.0) * range).round() as u16;
    println!("Pulse width: {}", pulse_width);
    pulse_width
}

pub fn write_to_pca(channel: u8, pulse_width: u16) -> Result<()> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(PCA9685_ADDR)?;

    // Only do this once ideally
    let prescale_val = ((25_000_000.0 / (4096.0 * PWM_FREQ_HZ)) - 1.0).round() as u8;
    i2c.smbus_write_byte(0x00, 0x10)?; // Sleep
    i2c.smbus_write_byte(0xFE, prescale_val)?; // Set prescale
    i2c.smbus_write_byte(0x00, 0x20)?; // Wake + auto-increment

    println!("Frequency set to {}", PWM_FREQ_HZ);
    let on_val: u16 = 0;
    let off_val = on_val + pulse_width;

    println!("On val: {}, off val: {}", on_val, off_val);

    let base_addr = 0x06 + 4 * channel;

    i2c.smbus_write_byte(base_addr, (on_val & 0xFF) as u8)?;         // LEDn_ON_L
    i2c.smbus_write_byte(base_addr + 1, (on_val >> 8) as u8)?;       // LEDn_ON_H
    i2c.smbus_write_byte(base_addr + 2, (off_val & 0xFF) as u8)?;    // LEDn_OFF_L
    i2c.smbus_write_byte(base_addr + 3, (off_val >> 8) as u8)?;      // LEDn_OFF_H

    println!("LED on/off L/H values written to smbus");

    Ok(())
}
