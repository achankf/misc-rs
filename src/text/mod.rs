mod buf;

pub struct Buf2 {
    row: usize,
    col: usize,
    crow: usize,
    ccol: usize,
    buf: Vec<char>,
}

#[macro_export]
macro_rules! bwrite_str {
	($buf:expr, $fmt:expr) => ($buf.write_str($fmt));
	($buf:expr, $fmt:expr, $($arg:tt)*) => ($buf.write_str(format!($fmt, $($arg)*).as_ref()));
}

#[macro_export]
macro_rules! bwrite_str_at {
	($buf:expr, $crow:expr, $ccol:expr, $fmt:expr) => ($buf.write_str_at($crow, $ccol, $fmt));
	($buf:expr, $crow:expr, $ccol:expr, $fmt:expr, $($arg:tt)*) => ($buf.write_str_at($crow, $ccol, format!($fmt, $($arg)*).as_ref()));
}
