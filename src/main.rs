#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::digital::{InputPin, OutputPin};


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut onboard_led = pins.d13.into_output();
    let mut cabin_light = pins.d12.into_output();
    let mut cabin_light_switch = pins.d2.into_pull_up_input();
    let message = "...---...";  // no joke, I need work.


    loop {
        cabin_light_controller(&mut cabin_light_switch, &mut cabin_light);
        mcode(&mut onboard_led, message);
    }
}

fn cabin_light_controller<PIn, POut>(input_pin: &mut PIn, output_pin: &mut POut)
where
    PIn: InputPin,
    POut: OutputPin,
{
    if input_pin.is_high().unwrap() {
        output_pin.set_high().unwrap();
    } else {
        output_pin.set_low().unwrap();
    }
}



fn blink_led<P>(led: &mut P)
where P: OutputPin,
{
    led.set_high()
        .unwrap();
    arduino_hal::delay_ms(500);
    led.set_low()
        .unwrap();
    arduino_hal::delay_ms(100);
    led.set_high()
        .unwrap();
    arduino_hal::delay_ms(500);
    led.set_low()
        .unwrap();
    arduino_hal::delay_ms(1000);
}


fn mcode<P>(led: &mut P, message: &str)
where P: OutputPin,
{
    for c in message.chars() {
        match c {
            '.' => {
                led.set_high().unwrap();
                arduino_hal::delay_ms(100);
                led.set_low().unwrap();
            }
            '-' => {
                led.set_high().unwrap();
                arduino_hal::delay_ms(300);
                led.set_low().unwrap();
            }
            ' ' => {
                arduino_hal::delay_ms(1000);
            }
            _ => {
                led.set_low().unwrap();
            }
        }
        arduino_hal::delay_ms(200);
    }
    arduino_hal::delay_ms(1000);
}