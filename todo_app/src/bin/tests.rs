#[derive(Debug)]
enum Modes {
    View,
    Add,
}

fn main() {

    let mut current_mode: Modes = Modes::View;

    change_mode_to_add(&mut current_mode);
    println!("Current mode: {:?}", current_mode);
}


fn change_mode_to_add(current_mode: &mut Modes) {
    *current_mode = Modes::Add;
}