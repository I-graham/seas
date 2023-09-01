#![windows_subsystem = "windows"]

mod game;
mod window;
fn main() {
    env_logger::init();
    game::play();
}
