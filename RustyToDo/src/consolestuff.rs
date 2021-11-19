use colored::Colorize;

pub fn print_menu() {
	clear();
	println!("=== Help Menu === \n");
	print_M_Line("a     ", "Add a new todo");
	print_M_Line("l     ", "List all todos");
	print_M_Line("c     ", "Change a todos Description");
	print_M_Line("mo    ", "Mark todo as open");
	print_M_Line("md    ", "Mark todo as done");
	print_M_Line("d     ", "Delete a todo");
	print_M_Line("dd    ", "Delete all done todos");
	print_M_Line("h/help", "Print this menu");
	print_M_Line("q     ", "Quit");
	println!("\n === Help Menu ===");
}

// <Danny> sc for shortcut, prints a menu line in color
pub fn print_M_Line(sc: &str, desc: &str) {
	println!("{} - {}", sc.blue(), desc.yellow());
}

pub fn clear() {
	print!("\x1B[2J\x1B[1;1H");
}

pub fn list_todo() {
	// List all todos in current list
	use console::Style;
	
	// <Danny> Color the done todos crossed out
	let crossed = Style::new().blink();
	println!("{}", crossed.apply_to("dude"))
}