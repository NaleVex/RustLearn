enum Modes {
    View,
    Add,
    Delete,
    Quit,
}

type Page = u16;

fn main() {
    println!("Program started");
    let mut todo_list: Vec<String> = Vec::new();
    let mut current_mode = Modes::View;
    let mut current_view_page: Page = 0;
    loop {
        println!("--------------------");
        match &current_mode {
            Modes::View => {
                println!("View mode");
                print_notes_of_page(&current_view_page, &todo_list);
            }
            Modes::Add => {
                println!("Add mode");
            }
            Modes::Delete => {
                println!("Delete mode");
            }
            Modes::Quit => {
                println!("Quit mode");
            }
        }
        print_hotkeys(&current_mode);
        let mut input_text = String::new();
        std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
        match current_mode {
            Modes::View => {
                match input_text.trim() {
                    "a" => current_mode = Modes::Add,
                    "d" => current_mode = Modes::Delete,
                    "q" => current_mode = Modes::Quit,
                    x if extract_number(x).is_some() => move_to_page(&x.parse::<Page>().unwrap() ,&todo_list ,&mut current_view_page),
                    _ => {}
                }
            }
            Modes::Add => {
                match input_text.trim() {
                    "v" => current_mode = Modes::View,
                    "q" => current_mode = Modes::Quit,
                    "d" => current_mode = Modes::Delete,
                    _ => add_note(&input_text, &mut todo_list),
                }
            }
            Modes::Delete => {
                match input_text.trim() {
                    "v" => current_mode = Modes::View,
                    "a" => current_mode = Modes::Add,
                    "q" => current_mode = Modes::Quit,
                    x if x.parse::<Page>().is_ok() => delete_note(&input_text, &mut todo_list, &current_view_page),
                    _ => {}
                }
            }
            Modes::Quit => {
                match input_text.trim() {
                    "q" => break,
                    "v" => current_mode = Modes::View,
                    "a" => current_mode = Modes::Add,
                    "d" => current_mode = Modes::Delete,
                    _ => {}
                }
            }
        }
    }
}


fn extract_number(text: &str) -> Option<i16> {
    text.parse::<i16>().ok()
}


fn print_notes_of_page(page: &Page, todo_list: &Vec<String>) {
    if todo_list.len() == 0 {
        println!("Nothing to show");
        return;
    }
    println!("Printing notes on page {}", page + 1);
    for i in 0..std::cmp::min(todo_list.len() - *page as usize * 10, 10) {
        println!("{}. {}", i+1, todo_list[*page as usize * 10 + i]);
    }
    println!("Page {} of {}", page, todo_list.len() / 10);
}


fn print_hotkeys(exclude: &Modes) {
    match *exclude {
        Modes::View => {
            println!("nums - view page, a - add, d - delete, q - quit");
        }
        Modes::Add => {
            println!("v - view list, d - delete, q - quit");
        }
        Modes::Delete => {
            println!("v - view list, a - add, q - quit");
        }
        Modes::Quit => {
            println!("q - really quit!, v - view list, a - add, d - delete");
        }
    }
}


fn move_to_page(page_to_go: &Page, todo_list: &Vec<String>, current_view_page: &mut Page) {
    let page_to_go: Page = page_to_go-1;
    let max_pages: Page = todo_list.len() as Page / 10;
    if page_to_go <= 0 {
       *current_view_page = 0;
    } else if page_to_go > max_pages {
        *current_view_page = max_pages;
    } else {
        *current_view_page = page_to_go;
    }
}


fn add_note(note: &String, todo_list: &mut Vec<String>) {
    if note.trim() == "" {
        println!("Empty note");
        return;
    }
    todo_list.push(note.to_string());
    println!("Note added");
}


fn delete_note(note: &String, todo_list: &mut Vec<String>, current_view_page: &Page) {
    let note_id_to_delete = note.trim().parse::<u16>().unwrap();
    // println!("Deleting note {note_id_to_delete}");
    if note_id_to_delete < 1 || 
    note_id_to_delete > 10 ||
    current_view_page * 10 + note_id_to_delete as Page - 1 >= todo_list.len() as Page {
        println!("Invalid note id");
    }
    else {
        todo_list.remove((current_view_page * 10 + note_id_to_delete - 1).into());
        println!("Note {} deleted.", note_id_to_delete);
    }
}