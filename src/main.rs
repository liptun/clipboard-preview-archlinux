use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

const LIMIT: usize = 16;

fn main() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let clipboard_content = clipboard.get_contents().unwrap_or_else(|_| "".to_string());
    let mut display_content = clipboard_content.chars().take(LIMIT).collect::<String>();
    if clipboard_content.len() > LIMIT {
        display_content.push_str("...");
    };
    println!("{}", display_content);
}

