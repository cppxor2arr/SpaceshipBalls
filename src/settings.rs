use std::fs::File;

use macroquad::input::KeyCode;
use ron::de;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub player1: Player,
    pub player2: Player,
    pub bullet: Bullet,
    pub shoot_cooldown: f32,
    pub restitution: f32,
}

#[derive(Deserialize)]
pub struct Player {
    #[serde(with = "KeyCodeDef")]
    pub up: KeyCode,
    #[serde(with = "KeyCodeDef")]
    pub down: KeyCode,
    #[serde(with = "KeyCodeDef")]
    pub left: KeyCode,
    #[serde(with = "KeyCodeDef")]
    pub right: KeyCode,
    #[serde(with = "KeyCodeDef")]
    pub shoot: KeyCode,
}

#[derive(Deserialize)]
pub struct Bullet {
    pub radius: f32,
    pub density: f32,
    pub speed: f32,
    pub spread: f32,
}

#[derive(Deserialize)]
#[serde(remote = "KeyCode")]
enum KeyCodeDef {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

pub fn load_settings() -> Settings {
    match File::open("config.ron") {
        Ok(f) => de::from_reader(f).unwrap_or_else(|err| {
            eprintln!("Failed to parse config.toml: {}", err);
            default_settings()
        }),
        Err(e) => {
            eprintln!("Failed to open config.toml: {}", e);
            default_settings()
        }
    }
}

pub fn default_settings() -> Settings {
    Settings {
        player1: Player {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            shoot: KeyCode::LeftShift,
        },
        player2: Player {
            up: KeyCode::P,
            down: KeyCode::Semicolon,
            left: KeyCode::L,
            right: KeyCode::Apostrophe,
            shoot: KeyCode::RightShift,
        },
        bullet: Bullet {
            radius: 0.1,
            density: 1.0,
            speed: 10.0,
            spread: 0.05,
        },
        shoot_cooldown: 0.1,
        restitution: 1.0,
    }
}
