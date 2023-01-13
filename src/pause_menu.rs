use std::process::Command;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

use crate::settings;

pub struct PauseMenu {
    pub active: bool,
}

impl Node for PauseMenu {
    fn ready(_node: RefMut<Self>)
    where
        Self: Sized,
    {
    }

    fn update(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        Command::new("gedit")
            .args(&["--wait", "config.ron"])
            .status()
            .expect("failed to run gedit");
        storage::store(settings::load_settings());
        node.active = false;
    }

    fn draw(_node: RefMut<Self>)
    where
        Self: Sized,
    {
    }
}
