pub fn printMenu() {
	clear();
	println!("\n ")
	printMLine("a", "Add a new todo");
	printMLine("l", "List all todos");
	printMLine("c", "Change a todos Description");
	printMLine("mo", "Mark todo as open");
	printMLine("md", "Mark todo as done");
	printMLine("d", "Delete a todo");
	printMLine("dd", "Delete all done todos");
	printMLine("h/help", "Print this menu");
	printMLine("q", "Quit")
}

// <Danny> sc for shortcut, prints a menu line in color
pub fn printMLine(sc: String, desc: String) {
	println!("{} - {}"), sc.blue(), desc.yellow();
}

pub fn clear() {
	print!("\x1B[2J\x1B[1;1H");
}