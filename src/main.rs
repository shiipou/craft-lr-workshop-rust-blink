#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32l4xx_hal::{delay::Delay, prelude::*, gpio::Edge, interrupt};

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
  rtt_init_print!();

  let core = cortex_m::Peripherals::take().unwrap();
  let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

  let mut flash = device.FLASH.constrain();
  let mut rcc = device.RCC.constrain();
  let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);

  let clocks = rcc.cfgr.sysclk(64.MHz()).pclk1(48.MHz()).freeze(&mut flash.acr, &mut pwr);

  let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
  let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
  let mut gpioc = device.GPIOC.split(&mut rcc.ahb2);

  let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
  let mut led2 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

  let mut timer = Delay::new(core.SYST, clocks);

  let mut userbtn = gpioc.pc13.into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);
  // Enable the interrupt on the button
  let mut syscfg = device.SYSCFG;
  userbtn.make_interrupt_source(&mut syscfg, &mut rcc.apb2);
  let mut dev_exti = device.EXTI;
  userbtn.enable_interrupt(&mut dev_exti);
  userbtn.trigger_on_edge(&mut dev_exti, Edge::Rising);

  rprintln!("Hello, world!");

  led1.set_low();
  led2.set_high();

  loop {
    led1.toggle();
    led2.toggle();

    rprintln!("toggle leds");
    timer.delay_ms(1000_u32);
  }
}

#[interrupt]
fn EXTI15_10() {
  rprintln!("Button pressed");

  let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();
  let mut gpioc = device.GPIOC.split(&mut device.RCC.constrain().ahb2);
  let mut led3 = gpioc.pc9.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
  let userbtn = gpioc.pc13.into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

  // Enable the interrupt in the NVIC
  cortex_m::interrupt::free(|cs| {
    if userbtn.is_low() {
      led3.set_low();
    } else {
      led3.set_high();
    }
  });
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
  rprintln!("panic : {}", panic);
  loop {}
}
