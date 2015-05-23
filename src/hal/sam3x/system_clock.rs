#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;

use core::option::Option::{self, Some, None};

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum RCFreq {
    MHz_4,
    MHz_8,
    MHz_12,
}

pub enum ClockSource {
    InternalSlow,
    InternalRC(RCFreq),
    Main(Option<u32>),
}

const SLOW_CLOCK_FREQ: u32 = 32_768;

impl ClockSource {
    fn init(&self) {
        use self::ClockSource::*;

        let css = match self {
            &InternalSlow => reg::PMC_mckr_css::SLOW_CLK,
            &InternalRC(freq) => {
                init_rc_oscillator(freq);
                reg::PMC_mckr_css::MAIN_CLK
            },
            &Main(_) => {
                // FIXME(mcoffin): Wait time hard coded
                init_main_oscillator(0x8);
                reg::PMC_mckr_css::MAIN_CLK
            }
        };

        // Switch master clock to chosen clock
        let pmc = &reg::PMC;
        pmc.mckr.set_css(css);
        wait_for!(pmc.st.mckrdy());
    }

    fn freq(&self) -> u32 {
        use self::ClockSource::*;
        use self::RCFreq::*;

        match self {
            &InternalSlow => SLOW_CLOCK_FREQ,
            &InternalRC(MHz_4) => 4_000_000,
            &InternalRC(MHz_8) => 8_000_000,
            &InternalRC(MHz_12) => 12_000_000,
            &Main(Some(f)) => f,
            &Main(None) => mck_freq(),
        }
    }
}

const MAINF_SCALE: u32 = 32_768 / 16;

/// Reads the main clock frequency from the PMC register.
///
/// NOTE: Will wait until the main frequency is measured
pub fn mck_freq() -> u32 {
    let pmc = &reg::PMC;

    wait_for!(pmc.mcfr.mainfrdy());
    let cycles = pmc.mcfr.mainf();
    cycles * MAINF_SCALE
}

pub struct Pll {
    pub mul: u32,
    pub div: u32,
    pub count: u32,
}

impl Pll {
    fn init(&self, src_freq: u32) {
        use self::reg::PMC_mckr_pres::*;
        let pmc = &reg::PMC;

        pmc.pllar.set_one(true).set_mula(self.mul)
            .set_diva(self.div).set_pllacount(self.count);
        wait_for!(pmc.st.locka());

        /*
        pmc.mckr.set_pres(match self.apply_freq(src_freq) {
            0 ... 84_000_000 => CLK,
            84_000_001 ... 168_000_000 => CLK_2,
            168_000_001 ... 336_000_000 => CLK_4,
            336_000_001 ... 672_000_000 => CLK_8,
            672_000_001 ... 1_344_000_000 => CLK_16,
            _ => panic!("Clock speed too fast!"),
        });
        */
        pmc.mckr.set_pres(CLK);
        wait_for!(pmc.st.mckrdy());

        pmc.mckr.set_css(reg::PMC_mckr_css::PLLA_CLK);
        wait_for!(pmc.st.mckrdy());
    }

    fn apply_freq(&self, freq: u32) -> u32 {
        (freq / self.div) * (self.mul + 1)
    }
}

/// Initializes the system master clock to be a given clock source optionally
/// with PLL scaling
pub fn init_clock(source: ClockSource, pll: Option<Pll>) -> u32 {
    source.init();

    let src_freq = source.freq();
    let freq = match pll {
        Some(ref p) => p.apply_freq(src_freq),
        _ => src_freq,
    };

    // Init flash
    init_flash(freq);

    match pll {
        Some(p) => p.init(src_freq),
        _ => {},
    }

    freq
}

/// Initializes the system exactly how CMSIS does (84MHz clock)
pub fn temp_boot() {
    use self::reg::PMC_mor_moscsel::*;
    use self::reg::PMC_mckr_css::*;
    use self::reg::PMC_mckr_pres::*;

    // Initialize flash
    let eefc0 = &reg::EEFC0;
    let eefc1 = &reg::EEFC1;
    eefc0.fmr.set_fws(4);
    eefc1.fmr.set_fws(4);

    // Enable main oscillator
    let pmc = &reg::PMC;
    match pmc.mor.moscsel() {
        MOSCXT => {},
        _ => {
            pmc.mor.set_key(MOR_KEY)
                .set_moscxtst(0x8)
                .set_moscrcen(true)
                .set_moscxten(true);
            wait_for!(pmc.st.moscxts());
        },
    }

    // Switch to Xtal oscillator
    pmc.mor.set_key(MOR_KEY)
        .set_moscxtst(0x8)
        .set_moscrcen(true)
        .set_moscxten(true)
        .set_moscsel(MOSCXT);
    wait_for!(pmc.st.moscsels());
    pmc.mckr.set_css(MAIN_CLK);
    wait_for!(pmc.st.mckrdy());

    // Initialize PLLA
    pmc.pllar.set_one(true)
        .set_mula(0x3)
        .set_pllacount(0x3f)
        .set_diva(0x1);
    wait_for!(pmc.st.locka());

    // Switch to main clock
    pmc.mckr.set_pres(CLK_2);
    wait_for!(pmc.st.mckrdy());

    // Switch to PLLA
    pmc.mckr.set_css(PLLA_CLK);
    wait_for!(pmc.st.mckrdy());

    ::hal::cortex_m3::systick::setup(24_000_000 / 1000);
    ::hal::cortex_m3::systick::enable();
}

static FLASH_MAX_FREQ: u32 = 20_000_000;

fn init_flash(clk_freq: u32) {
    let eefc0 = &reg::EEFC0;
    let eefc1 = &reg::EEFC1;

    let cycles: u32 = clk_freq / FLASH_MAX_FREQ;
    eefc0.fmr.set_fws(cycles);
    eefc1.fmr.set_fws(cycles);

    wait_for!(eefc0.fsr.fready() &&
              eefc1.fsr.fready());
}

static MOR_KEY: u32 = 0x37;

fn init_rc_oscillator(freq: RCFreq) {
    let pmc = &reg::PMC;

    // Enable MOSCRC
    pmc.mor.set_moscrcen(true).set_moscrcf(match freq {
        RCFreq::MHz_4 => reg::PMC_mor_moscrcf::MHz_4,
        RCFreq::MHz_8 => reg::PMC_mor_moscrcf::MHz_8,
        RCFreq::MHz_12 => reg::PMC_mor_moscrcf::MHz_12,
    }).set_key(MOR_KEY);
    wait_for!(pmc.st.moscrcs());

    // Switch main clock to MOSCRC
    pmc.mor.set_moscsel(reg::PMC_mor_moscsel::MOSCRC).set_key(MOR_KEY);
    wait_for!(pmc.st.moscsels());
}

fn init_main_oscillator(start_time: u32) {
    let pmc = &reg::PMC;

    // Enable MOSCXT
    pmc.mor.set_moscxten(true).set_moscrcen(true)
        .set_moscxtst(start_time).set_key(MOR_KEY);
    wait_for!(pmc.st.moscxts());

    // Switch main clock to MOSCXT
    pmc.mor.set_moscsel(reg::PMC_mor_moscsel::MOSCXT).set_key(MOR_KEY);
    wait_for!(pmc.st.moscsels());
}

mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(PMC = {
        0x20 => reg32 mor {
            0 => moscxten,
            1 => moscxtby,
            3 => moscrcen,
            4..6 => moscrcf {
                0 => MHz_4,
                1 => MHz_8,
                2 => MHz_12
            },
            8..15 => moscxtst,
            16..23 => key,
            24 => moscsel {
                0 => MOSCRC,
                1 => MOSCXT
            },
            25 => cfden
        }
        0x24 => reg32 mcfr {
            0..15 => mainf: ro,
            16 => mainfrdy: ro
        }
        0x28 => reg32 pllar {
            0..7 => diva,
            8..13 => pllacount,
            16..26 => mula,
            29 => one
        }
        0x30 => reg32 mckr {
            0..1 => css {
                0 => SLOW_CLK,
                1 => MAIN_CLK,
                2 => PLLA_CLK,
                3 => UPLL_CLK
            },
            4..6 => pres {
                0 => CLK,
                1 => CLK_2,
                2 => CLK_4,
                3 => CLK_8,
                4 => CLK_16,
                5 => CLK_32,
                6 => CLK_64,
                7 => CLK_3
            },
            12 => plladiv2,
            13 => uplldiv2
        }
        0x68 => reg32 st {
            0 => moscxts: ro,
            1 => locka: ro,
            3 => mckrdy: ro,
            16 => moscsels: ro,
            17 => moscrcs: ro,
        }
    });

    ioregs!(EEFC = {
        0x0 => reg32 fmr {
            0..11 => fws
        }
        0x8 => reg32 fsr {
            0 => fready: ro
        }
    });

    extern {
        #[link_name="sam3x_iomem_PMC"]
        pub static PMC: PMC;
        #[link_name="sam3x_iomem_EEFC0"]
        pub static EEFC0: EEFC;
        #[link_name="sam3x_iomem_EEFC1"]
        pub static EEFC1: EEFC;
    }
}
