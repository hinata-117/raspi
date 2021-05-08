/**
 * Raspberry pi GPIO sample program
 * @file gpio.rs
 * @author hinata
 * @date 2021/05/08 created
 */
mod raspi;
use crate::raspi::gpio::Gpio;
use crate::raspi::gpio::PioMode;
use crate::raspi::gpio::GpioPin;

fn main() {
  let mut gpio = Gpio::new();

  if let Err(ret) = gpio.initialize()
  {
    println!("{:?}", ret);
  }
  else
  {
    gpio.set_pio_mode(GpioPin::Gpio17, PioMode::Output);
    gpio.write_data(GpioPin::Gpio17, true);
    println!("GPIO17 LED ON success\n");
  }
}

