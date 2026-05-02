#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

/// First byte of application flash — cortex-m-rt places the vector table here.
const APP_BASE: u32 = 0x0800_8000;

/// Valid SRAM bounds for STM32F401RE (96 KB).
const SRAM_START: u32 = 0x2000_0000;
const SRAM_END:   u32 = 0x2001_8000;

#[entry]
fn main() -> ! {
    // The first two words of any Cortex-M vector table are:
    //   [0]: initial Main Stack Pointer value
    //   [1]: Reset_Handler address (Thumb bit = LSB must be 1)
    let app_sp    = read_word(APP_BASE);
    let app_reset = read_word(APP_BASE + 4);

    if is_valid(app_sp, app_reset) {
        unsafe { boot(app_sp, app_reset) }
    }

    // No valid application image found.
    // In a real system: enter firmware-update mode via UART/USB.
    // Here we simply halt — the debugger will show where we stopped.
    loop {
        cortex_m::asm::nop();
    }
}

/// Sanity-check the application's vector table before jumping.
///
/// A freshly erased sector reads as 0xFFFF_FFFF.  Checking the SP and
/// reset vector catches the "nothing is flashed" case before we jump to
/// a random address and corrupt everything.
fn is_valid(sp: u32, reset: u32) -> bool {
    // SP must be inside SRAM (stack starts at top, grows down)
    let sp_ok = sp > SRAM_START && sp <= SRAM_END;

    // Reset vector must point into application flash.
    // Strip the Thumb bit (LSB=1 is required by ARMv7-M, not part of the address).
    let reset_addr = reset & !1u32;
    let reset_ok = reset_addr >= APP_BASE && reset_addr < 0x0808_0000;

    sp_ok && reset_ok
}

/// Relocate the vector table and hand control to the application.
///
/// After this function executes, the bootloader is gone from the call stack.
/// We set the MSP from the application's vector table, then branch to its
/// Reset_Handler.  Using `options(noreturn)` tells the compiler there is no
/// path back, so no epilogue is emitted.
#[inline(never)]
unsafe fn boot(sp: u32, reset_handler: u32) -> ! {
    // Point VTOR to the application's vector table so that all subsequent
    // exceptions and interrupts dispatch through the application's handlers.
    const SCB_VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
    core::ptr::write_volatile(SCB_VTOR, APP_BASE);

    // DSB + ISB: ensure VTOR write and pipeline flush complete before the
    // branch, as required by the ARM architecture reference manual.
    core::arch::asm!("dsb", "isb", options(nostack));

    core::arch::asm!(
        "msr msp, {sp}",   // load application stack pointer into MSP
        "bx  {reset}",     // branch to application Reset_Handler (Thumb)
        sp    = in(reg) sp,
        reset = in(reg) reset_handler,
        options(noreturn),
    );
}

fn read_word(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}
