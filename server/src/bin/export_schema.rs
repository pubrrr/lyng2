use lyng2::chat::build_schema;
use std::io::Write;

fn main() {
    std::io::stdout()
        .write_all(build_schema().sdl().as_bytes())
        .unwrap();
}
