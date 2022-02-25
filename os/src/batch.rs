use crate::config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};
use crate::trap::TrapContext;

static KERNEL_STACK: [u8; KERNEL_STACK_SIZE] = [0; KERNEL_STACK_SIZE];
static USER_STACK: [u8; USER_STACK_SIZE] = [0; USER_STACK_SIZE];

fn test_app() {
    println!("From user space!");
    loop {}
}

pub fn run_next_app() -> ! {
    let context = TrapContext::app_init_context(test_app as _, USER_STACK.as_ptr_range().end as _);
    unsafe { context.exec(KERNEL_STACK.as_ptr_range().end as _) };
}