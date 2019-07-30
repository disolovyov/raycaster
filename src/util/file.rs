use futures::Future;
use quicksilver::load_file;

pub fn load_file_sync(name: &str) -> Vec<u8> {
    load_file(name)
        .wait()
        .expect(&format!("Failed to load '{}'", name))
}
