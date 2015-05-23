#![allow(dead_code)]

use core::option::Option::{self, Some};

extern {
    fn isr_irq0();
    fn isr_irq1();
    fn isr_irq2();
    fn isr_irq3();
    fn isr_irq4();
    fn isr_irq5();
    fn isr_irq6();
    fn isr_irq7();
    fn isr_irq8();
    fn isr_irq9();
    fn isr_irq10();
    fn isr_irq11();
    fn isr_irq12();
    fn isr_irq13();
    fn isr_irq14();
    fn isr_irq15();
    fn isr_irq16();
    fn isr_irq17();
    fn isr_irq18();
    fn isr_irq19();
    fn isr_irq20();
    fn isr_irq21();
    fn isr_irq22();
    fn isr_irq23();
    fn isr_irq24();
    fn isr_irq25();
    fn isr_irq26();
    fn isr_irq27();
    fn isr_irq28();
    fn isr_irq29();
}

#[allow(non_upper_case_globals)]
const ISRCount: usize = 30;

#[allow(non_upper_case_globals)]
#[link_section=".isr_vector_nvic"]
#[no_mangle]
pub static NVICVectors: [Option<unsafe extern fn()>; ISRCount] = [
    Some(isr_irq0),
    Some(isr_irq1),
    Some(isr_irq2),
    Some(isr_irq3),
    Some(isr_irq4),
    Some(isr_irq5),
    Some(isr_irq6),
    Some(isr_irq7),
    Some(isr_irq8),
    Some(isr_irq9),
    Some(isr_irq10),
    Some(isr_irq11),
    Some(isr_irq12),
    Some(isr_irq13),
    Some(isr_irq14),
    Some(isr_irq15),
    Some(isr_irq16),
    Some(isr_irq17),
    Some(isr_irq18),
    Some(isr_irq19),
    Some(isr_irq20),
    Some(isr_irq21),
    Some(isr_irq22),
    Some(isr_irq23),
    Some(isr_irq24),
    Some(isr_irq25),
    Some(isr_irq26),
    Some(isr_irq27),
    Some(isr_irq28),
    Some(isr_irq29),
    /*
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    */
];
