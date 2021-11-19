use todo::*;
use std::env;

use crate::consolestuff::list_todo;

mod todo;
mod consolestuff;


fn main() {
    use std::env;
    use todo::*;
    use consolestuff::*;
    
    let mut input_mode: bool = true;
    let mut todo_list: Vec<ToDo> = Vec::new();
    // <Danny> Getting all arguments from start
    let mut args: Vec<String> = Vec::new(); 
    
    
    list_todo();
    args = env::args().collect();
    // <Danny> If input mode == true, then nothing will be printed, unless it is required in the
    if args.len() >= 2  {
        input_mode = false;
    }

    // <Danny> Starting with looking for a database file 
    // <!TODO> Create file check

    // <Danny> If file found -> Load stuff into Vec
    // <!TODO> File content into Vec
    let mut input: String = String::new();
    print_menu();
    input = get_in();
    match input.as_str() {
        "a" => {
            // <Danny> If input_mode != true, don't display messages
            if input_mode == true {
            print!("[?] please enter new todo: ");
            input = get_in();
            } else {
            input = args[2]
            }
            let new_todo: ToDo;
            new_todo.status = Status::open;
            new_todo.desc = input;

            todo_list.push(new_todo);
            todo_list.sort_by_key(|x|x.status);
            list_todo()
        },
        "c" => {},
        "l" => {},
        "mo" => {},
        "d" => {},
        "dd" => {},
        "h" | "help" => {},
        "q" => {},
        _ => {}
    }
}

fn get_in() -> String{
    let mut input: String;
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut ausl = std::io::stdin().read_line(&mut input).unwrap();
    return input;
}