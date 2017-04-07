extern crate libc;

mod editor;
mod terminal;

fn main() {
    terminal::enable_raw_mode();
    let mut editor = editor::Editor::new();
    loop {
        editor.refresh_screen();
        editor.process_keypress();
    }
}
