#![no_std]
#![no_main]

// Neopixel Rainbow
// This only functions when the --release version is compiled. Using the debug
// version leads to slow pulse durations which results in a straight white LED
// output.
//
// // Needs to be compiled with --release for the timing to be correct

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use circuit_playground_express as bsp;
use bsp::hal;
use bsp::pac;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::timer::TimerCounter;
use pac::{CorePeripherals, Peripherals};

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8, SmartLedsWrite,
};

use ws2812_timer_delay as ws2812;
use ws2812::Ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());

    let neopixel_pin: bsp::NeoPixel = pins.d8.into_push_pull_output();
    let mut neopixel = Ws2812::new(timer, neopixel_pin);

    // Loop through all of the available hue values (colors) to make a
    // rainbow effect from the onboard neopixel
    // loop {
    //     for j in 0..255u8 {
    //         let colors = [hsv2rgb(Hsv {
    //             hue: j,
    //             sat: 255,
    //             val: 2,
    //         }); 10];
    //     }
    //     neopixel.write(colors.iter().cloned()).unwrap();
    //     delay.delay_ms(5u8);
    // }

    let black = [RGB8 { r: 0, g: 0, b: 0 }; 10];
    
    let mut i: usize = 0;
    let mut increasing = true;
    let mut target_color = generate_color(100);
    let mut send_target = black;
    
    loop {
        let target_hue = ((255 * i) / 10) as u8; // Evenly space 10 colors around 255 values
        target_color = generate_color(target_hue);
        send_target[i as usize] = target_color;
        neopixel.write(send_target.iter().cloned()).unwrap();
        delay.delay_ms(50u8);

        if increasing {
            i += 1; 
            if i >= 9 {
                increasing = false;
                send_target = black;
                neopixel.write(black.iter().cloned()).unwrap();
            }
        } else {
            i -= 1;
            if i <= 0 {
                increasing = true;
                send_target = black;
                neopixel.write(black.iter().cloned()).unwrap();
            }
        }
    }

}

fn generate_color(hue: u8) -> RGB8 {
    hsv2rgb(Hsv {
        hue,
        sat: 255,
        val: 2,
    })
}