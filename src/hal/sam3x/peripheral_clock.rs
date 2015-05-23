#[derive(Clone, Copy)]
pub struct PeripheralClock {
    pub index: u8,
}

fn get_pmc() -> &'static reg::PMC {
    &reg::PMC_I
}

enum Group {
    Group0,
    Group1,
}

impl PeripheralClock {
    pub fn new(index: u8) -> PeripheralClock {
        PeripheralClock {
            index: index,
        }
    }

    pub fn enable(&self) {
        match self.group_info() {
            (Group::Group0, idx) => {
                get_pmc().pcer0.set_pce(idx as usize, true);
            },
            (Group::Group1, idx) => {
                get_pmc().pcer1.set_pce(idx as usize, true);
            },
        };
    }

    pub fn disable(&self) {
        match self.group_info() {
            (Group::Group0, idx) => {
                get_pmc().pcdr0.set_pcd(idx as usize, true);
            },
            (Group::Group1, idx) => {
                get_pmc().pcdr1.set_pcd(idx as usize, true);
            },
        };
    }

    fn group_info(&self) -> (Group, u8) {
        match self.index {
            0...31 => (Group::Group0, self.index),
            32...44 => (Group::Group1, self.index - 32),
            _ => panic!("Peripheral Clock index out of range"),
        }
    }
}

mod reg {
    use core::ops::Drop;
    use volatile_cell::VolatileCell;

    ioregs!(PMC = {
        0x10 => reg32 pcer0 {
            0..31 => pce[32]: wo,
        }
        0x14 => reg32 pcdr0 {
            0..31 => pcd[32]: wo,
        }
        0x18 => reg32 pcsr0 {
            0..31 => pcs[32]: ro,
        }
        0x100 => reg32 pcer1 {
            0..31 => pce[32]: wo,
        }
        0x104 => reg32 pcdr1 {
            0..31 => pcd[32]: wo,
        }
        0x108 => reg32 pcsr1 {
            0..31 => pcs[32]: ro,
        }
    });

    extern {
        #[link_name="sam3x_iomem_PMC"]
        pub static PMC_I: PMC;
    }
}
