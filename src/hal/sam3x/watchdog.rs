//! Watchdog timer configuration

/// Enables the watchdog timer
pub fn watchdog_enable() {
    let wdc = &reg::WDC;
    wdc.mr.set_wddis(false);
}

/// Disables the watchdog timer
pub fn watchdog_disable() {
    let wdc = &reg::WDC;
    wdc.mr.set_wddis(true);
}

static WDT_CR_KEY: u32 = 0xa5;

/// Resets the watchdog timer
pub fn watchdog_feed() {
    let wdc = &reg::WDC;
    wdc.cr.set_wdrstt(true).set_key(WDT_CR_KEY);
}

mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(WDC = {
        0x0 => reg32 cr {
            0 => wdrstt: wo,
            24..31 => key: wo
        }
        0x4 => reg32 mr {
            15 => wddis
        }
    });

    extern {
        #[link_name="sam3x_iomem_WDC"]
        pub static WDC: WDC;
    }
}
