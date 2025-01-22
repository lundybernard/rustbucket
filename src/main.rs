#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::digital::OutputPin;

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
    let mut external_led = pins.d12.into_output();

    loop {
        blink_led(&mut onboard_led);
        blink_led(&mut external_led);
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