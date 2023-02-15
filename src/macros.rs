// macro_rules! align_down {
// 	($value:expr, $alignment:expr) => {
// 		$value & !($alignment - 1)
// 	};
// }

// macro_rules! align_up {
// 	($value:expr, $alignment:expr) => {
// 		align_down!($value + ($alignment - 1), $alignment)
// 	};
// }

/// Print formatted text to our console.
///
/// From http://blog.phil-opp.com/rust-os/printing-to-screen.html, but tweaked
/// for HermitCore.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::_print(::core::format_args!($($arg)*));
    }};
}

/// Print formatted text to our console, followed by a newline.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        print!("{}\n", ::core::format_args!($($arg)*))
    }};
}
