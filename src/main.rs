extern crate gpio_cdev;

use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

use gpio_cdev::{Chip, errors, LineRequestFlags};

fn main() -> errors::Result<()> {
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let red_led = chip.get_line(4)?.request(LineRequestFlags::OUTPUT, 0, "rpi-ledmatrix-rs")?;
    let yellow_led = chip.get_line(18)?.request(LineRequestFlags::OUTPUT, 0, "rpi-ledmatrix-rs")?;
    let green_led = chip.get_line(23)?.request(LineRequestFlags::OUTPUT, 0, "rpi-ledmatrix-rs")?;
    let blue_led = chip.get_line(24)?.request(LineRequestFlags::OUTPUT, 0, "rpi-ledmatrix-rs")?;

    let leds = [red_led, yellow_led, green_led, blue_led];

    let duration = Duration::from_millis(60_000);
    let interval = Duration::from_millis(100);
    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        sleep(interval);
        for led in &leds {
            led.set_value(0)?;
        }
        sleep(interval);
        for led in &leds {
            led.set_value(1)?;
        }
    }

    Ok(())
}
