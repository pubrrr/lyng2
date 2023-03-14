use std::io::Write;

use lyng2::chat::build_schema;
use lyng2::chat::repository::InMemoryRepository;

fn main() {
    std::io::stdout()
        .write_all(build_schema::<InMemoryRepository>().sdl().as_bytes())
        .unwrap();
}
