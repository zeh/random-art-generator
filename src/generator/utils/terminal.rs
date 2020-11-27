// More info: https://en.wikipedia.org/wiki/ANSI_escape_code

pub fn cursor_up() {
	print!("\u{1b}[1;A");
}

pub fn erase_line_to_end() {
	print!("\u{1b}[0;K");
}
