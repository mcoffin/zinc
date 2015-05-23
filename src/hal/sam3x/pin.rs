use super::peripheral_clock;
pub use self::Controller::*;
use hal::pin;
use hal::pin::{Gpio, GpioConf};

use core::convert::From;

/// Available controllers
#[derive(Clone, Copy)]
pub enum Controller {
    ControllerA,
    ControllerB,
    ControllerC,
    ControllerD,
    ControllerE,
    ControllerF,
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Peripheral {
    PeripheralA,
    PeripheralB,
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Function {
    Gpio(pin::GpioDirection),
    Peripheral(Peripheral),
}

impl Controller {
    fn to_reg(self) -> &'static reg::PIO {
        match self {
            ControllerA => &reg::PIO_A,
            ControllerB => &reg::PIO_B,
            ControllerC => &reg::PIO_C,
            ControllerD => &reg::PIO_D,
            ControllerE => &reg::PIO_E,
            ControllerF => &reg::PIO_F,
        }
    }

    fn clock(self) -> peripheral_clock::PeripheralClock {
        let index = match self {
            ControllerA => 11,
            ControllerB => 12,
            _ => unimplemented!(),
        };
        peripheral_clock::PeripheralClock {
            index: index,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Pin {
    pub controller: Controller,
    pub pin: u8,
}

impl Pin {
    pub fn new(controller: Controller, pin_index: u8,
               function: Function) -> Pin {
        let p = Pin {
            controller: controller,
            pin: pin_index,
        };
        let offset = p.pin as usize;
        let pio = p.controller.to_reg();

        p.start_clock();

        match function {
            Function::Gpio(direction) => {
                pio.per.set_pe(offset, true);
                p.set_direction(direction);
            },
            Function::Peripheral(peripheral) => {
                pio.pdr.set_pd(offset, true);
                pio.absr.set_abs(offset, From::from(peripheral));
            },
        }

        p
    }

    fn start_clock(&self) {
        let p_clk = self.controller.clock();
        p_clk.enable();
    }
}

impl From<GpioConf> for Pin {
  fn from(c: GpioConf) -> Pin {
    let idx = (c.index % 32) as u8;
    let controller = match c.index / 32 {
      0 => ControllerA,
      1 => ControllerB,
      2 => ControllerC,
      3 => ControllerD,
      4 => ControllerE,
      5 => ControllerF,
      _ => panic!(),
    };

    Pin::new(controller,
             idx,
             Function::Gpio(c.direction))
  }
}

impl ::hal::pin::Gpio for Pin {
    fn set_high(&self) {
        let pio = self.controller.to_reg();

        pio.sodr.set_sod(self.pin as usize, true);
    }

    fn set_low(&self) {
        let pio = self.controller.to_reg();

        pio.codr.set_cod(self.pin as usize, true);
    }

    fn level(&self) -> ::hal::pin::GpioLevel {
        let pio = self.controller.to_reg();

        match pio.pdsr.pds(self.pin as usize) {
            false => ::hal::pin::Low,
            _ => ::hal::pin::High,
        }
    }

    fn set_direction(&self, new_dir: ::hal::pin::GpioDirection) {
        let pio = self.controller.to_reg();
        let offset = self.pin as usize;

        match new_dir {
            ::hal::pin::In => {
                pio.odr.set_od(offset, true);
            },
            ::hal::pin::Out => {
                pio.oer.set_oe(offset, true);
            },
        }
    }
}



mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(PIO = {
        0x0 => reg32 per {
            0..31 => pe[32]: wo
        }
        0x4 => reg32 pdr {
            0..31 => pd[32]: wo
        }
        0x8 => reg32 psr {
            0..31 => ps[32]: ro
        }
        0x10 => reg32 oer {
            0..31 => oe[32]: wo
        }
        0x14 => reg32 odr {
            0..31 => od[32]: wo
        }
        0x18 => reg32 osr {
            0..31 => os[32]: ro
        }
        0x20 => reg32 ifer {
            0..31 => ife[32]: wo
        }
        0x24 => reg32 ifdr {
            0..31 => ifd[32]: wo
        }
        0x28 => reg32 ifsr {
            0..31 => ifd[32]: ro
        }
        0x30 => reg32 sodr {
            0..31 => sod[32]: wo
        }
        0x34 => reg32 codr {
            0..31 => cod[32]: wo
        }
        0x38 => reg32 odsr {
            0..31 => ods[32]
        }
        0x3c => reg32 pdsr {
            0..31 => pds[32]: ro
        }
        0x40 => reg32 ier {
            0..31 => ie[32]: wo
        }
        0x44 => reg32 idr {
            0..31 => id[32]: wo
        }
        0x48 => reg32 imr {
            0..31 => im[32]: ro
        }
        0x4c => reg32 isr {
            0..31 => is[32]: ro
        }
        0x50 => reg32 mder {
            0..31 => mde[32]: wo
        }
        0x54 => reg32 mddr {
            0..31 => mdd[32]: wo
        }
        0x58 => reg32 mdsr {
            0..31 => mds[32]: ro
        }
        0x60 => reg32 pudr {
            0..31 => pud[32]: wo
        }
        0x64 => reg32 puer {
            0..31 => pue[32]: wo
        }
        0x68 => reg32 pusr {
            0..31 => pus[32]: ro
        }
        0x70 => reg32 absr {
            0..31 => abs[32]: rw {
                0 => PeripheralA,
                1 => PeripheralB
            }
        }
    });

    use super::Peripheral;
    use core::convert::From;

    impl From<Peripheral> for PIO_absr_abs {
        fn from(p: Peripheral) -> PIO_absr_abs {
            match p {
                Peripheral::PeripheralA => PIO_absr_abs::PeripheralA,
                Peripheral::PeripheralB => PIO_absr_abs::PeripheralB,
            }
        }
    }

    extern {
        #[link_name="sam3x_iomem_PIOA"]
        pub static PIO_A: PIO;
        #[link_name="sam3x_iomem_PIOB"]
        pub static PIO_B: PIO;
        #[link_name="sam3x_iomem_PIOC"]
        pub static PIO_C: PIO;
        #[link_name="sam3x_iomem_PIOD"]
        pub static PIO_D: PIO;
        #[link_name="sam3x_iomem_PIOE"]
        pub static PIO_E: PIO;
        #[link_name="sam3x_iomem_PIOF"]
        pub static PIO_F: PIO;
    }
}
