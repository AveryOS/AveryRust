use core::prelude::*;

const VGA: *mut u16 = 0xb8000 as *mut u16;

const SIZE_X: int = 80;
const SIZE_Y: int = 25;

const MIN_X: int = 2;
const MIN_Y: int = 1;
const MAX_X: int = 78;
const MAX_Y: int = 24;

const COLOR: u16 = (0 << 8) | (7 << 12);

static mut x: int = MIN_X;
static mut y: int = MIN_Y;

unsafe fn update_cursor()
{
	use arch::outb;

	let loc = y * SIZE_X + x;
   
	outb(0x3D4, 14);
	outb(0x3D5, (loc >> 8) as u8);
	outb(0x3D4, 15);
	outb(0x3D5, loc as u8);
}

pub fn scroll() {
	unsafe {
		for i in range(SIZE_X, SIZE_X * (SIZE_Y - 1)) {
			*VGA.offset(i) = *VGA.offset(i + SIZE_X);
		}

		for i in range(0, SIZE_X) {
			*VGA.offset((SIZE_Y - 1) * SIZE_X + i) = ' ' as u16 | COLOR;
		}
	}
}

pub fn cls() {
	unsafe {
		for i in range(0, SIZE_X * SIZE_Y) {
			*VGA.offset(i) = ' ' as u16 | COLOR;
		}

		x = MIN_X;
		y = MIN_Y;

		update_cursor();
	}

}

pub fn newline() {
	unsafe {
		y += 1;
		x = MIN_X;

		if y >= MAX_Y {
			scroll();
			y = MAX_Y - 1;
		}

		update_cursor();
	}
}

pub fn putc(c: char) {
	unsafe {

		match c {
			'\n' => newline(),
			'\t' => {
				x = (x + 4) & !(4 - 1);

				if x >= MAX_X {
					newline();
				} else {
					update_cursor();
				}
			}
			_ => {
				if x >= MAX_X {
					newline();
				}

				*VGA.offset(y * SIZE_X + x) = c as u16 | COLOR;
				x += 1;
				update_cursor();
			}
		}

	}
}
