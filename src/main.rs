use macroquad::{
    color,
    experimental::scene::{self},
    input::{self, KeyCode},
    window,
};

mod bullet;
mod components;
mod constants;
mod game;
mod init_world;
mod input_system;
mod particles;
mod pause_menu;
mod physics;
mod random;
mod render_system;
mod renderer;
mod settings;
mod update_system;
mod utils;

use crate::{game::Game, utils::window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = scene::add_node(Game::new());

    loop {
        utils::handle_resize();

        if scene::get_node(game).is_none() {
            break;
        }
        if input::is_key_pressed(KeyCode::F5) {
            scene::get_node(game).unwrap().delete();
            game = scene::add_node(Game::new());
        }

        window::clear_background(color::BLACK);
        scene::update();
        window::next_frame().await;
    }
}
