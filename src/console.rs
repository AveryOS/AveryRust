use std::fmt::{Write, Arguments, Error};
use spin::Mutex;

pub use arch;

static LOCK: Mutex<()> = Mutex::new(());

struct ScreenWriter;

impl Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
		for c in s.chars() {
			arch::console::putc(c);
		}

		Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => (
        ::console::print_args(format_args!($($arg)*))
    )
}

macro_rules! println {
    ($($arg:tt)*) => (
        ::console::println_args(format_args!($($arg)*))
    )
}

pub fn println_args(args: Arguments) {
    let _ = LOCK.lock();
    assert!(ScreenWriter.write_fmt(args).is_ok());
    arch::console::putc('\n');
}

pub fn print_args(args: Arguments) {
    let _ = LOCK.lock();
	assert!(ScreenWriter.write_fmt(args).is_ok());
}

#[lang = "eh_personality"]
extern fn eh_personality()
{
    panic!("Exceptions not supported");
}

#[allow(unreachable_code)]
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: Arguments, file: &'static str, line: u32) -> ! {

    println!("\nPanic: {}\nLoc: {}:{}", fmt, file, line);

    unsafe {
        arch::panic();

        static mut TRIED_BACKTRACE: bool = false;

        if !TRIED_BACKTRACE {
            TRIED_BACKTRACE = true;
            arch::symbols::print_backtrace();
            print!("@@@");
        } else {
            print!("Panic during backtrace...");
        }

    	arch::panic();
    }
}
