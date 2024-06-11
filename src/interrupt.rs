use crate::hardware_interrupt;
use crate::hardware_interrupt::InterruptIndex;
use crate::println;
use crate::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// Defining the IDT.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(hardware_interrupt::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(hardware_interrupt::keyboard_interrupt_handler);
        // Return the struct.
        idt
    };
}

// Load the Interrupt Descriptor Table
pub fn init_idt(){
    IDT.load();
}

// BreakPoint Handler
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame : InterruptStackFrame
){
    println!("EXCEPTION BREAKPOINT\n{:#?}",stack_frame)
}

// Double Fault Handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame : InterruptStackFrame,
    _error_code: u64
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}