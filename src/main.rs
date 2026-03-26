// main.rs

#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points



use core::panic::PanicInfo;

mod vga_buffer;
use vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT};

// This function is called on Panic:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const POTATO_FRAMES: &[&[&str]] = &[
    &[
        "  ___  ",
        " (o o) ",
        "  \\_/  ",
    ],
    &[
        "  ___  ",
        " (o_o) ",
        "  \\_/  ",
    ],
    &[
        "  ___  ",
        " (O o) ",
        "  \\_/  ",
    ],
    &[
        "  ___  ",
        " (o O) ",
        "  \\_/  ",
    ],
];

fn delay(count: u64) {
    for _ in 0..count {
        // safe inline nop
        unsafe { core::arch::asm!("nop"); }
    }
}

/*
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("#Output").unwrap();
    write!(vga_buffer::WRITER.lock(), ", integer, float test {} and {}", 42, 1.87).unwrap();
    
    loop {}
}
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    let mut writer = vga_buffer::WRITER.lock();

    // 1) Title at row 0, centered-ish
    let title = "Rotato the Potato";
    let col = (BUFFER_WIDTH - title.len()) / 2;
    writer.write_at(0, col, title);

    // 2) Animation loop
    let frame_count = POTATO_FRAMES.len();
    let region_row = 2;
    let region_col = (BUFFER_WIDTH - 7) / 2;  // assuming 7 columns width
    let region_rows = 3;
    let region_cols = 7;
    let delay_amount = 10_000_000;  // tune this for speed

    let mut idx = 0;
    loop {
        // clear previous
        writer.clear_region(region_row, region_rows, region_col, region_cols);

        // draw next frame
        let frame = POTATO_FRAMES[idx];
        for (i, line) in frame.iter().enumerate() {
            writer.write_at(region_row + i, region_col, line);
        }

        // advance and wrap
        idx = (idx + 1) % frame_count;

        // pause so we can see it spin
        delay(delay_amount);
    }
}