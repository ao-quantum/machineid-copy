use clipboard::ClipboardProvider;

fn main() {
    clipboard::ClipboardContext::set_contents(&mut (ClipboardProvider::new().unwrap()), machine_uid::get().unwrap()).unwrap();
}
