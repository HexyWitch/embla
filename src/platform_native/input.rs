use input::{Key, MouseButton};
use sdl2::keyboard::Keycode as Sdl2Keycode;
use sdl2::mouse::MouseButton as Sdl2MouseButton;

pub fn to_mouse_button(button: Sdl2MouseButton) -> MouseButton {
    match button {
        Sdl2MouseButton::Unknown => MouseButton::Unknown,
        Sdl2MouseButton::Left => MouseButton::Left,
        Sdl2MouseButton::Middle => MouseButton::Middle,
        Sdl2MouseButton::Right => MouseButton::Right,
        _ => MouseButton::Unknown,
    }
}

pub fn to_key(key_code: Sdl2Keycode) -> Key {
    match key_code {
        Sdl2Keycode::Backspace => Key::Backspace,
        Sdl2Keycode::Space => Key::Space,
        Sdl2Keycode::Tab => Key::Tab,
        Sdl2Keycode::Return => Key::Return,
        Sdl2Keycode::LShift => Key::Shift,
        Sdl2Keycode::LCtrl => Key::Ctrl,
        Sdl2Keycode::LAlt => Key::Alt,
        Sdl2Keycode::RShift => Key::Shift,
        Sdl2Keycode::RCtrl => Key::Ctrl,
        Sdl2Keycode::RAlt => Key::Alt,
        Sdl2Keycode::Pause => Key::Pause,
        Sdl2Keycode::CapsLock => Key::CapsLock,
        Sdl2Keycode::Escape => Key::Escape,
        Sdl2Keycode::PageUp => Key::PageUp,
        Sdl2Keycode::PageDown => Key::PageDown,
        Sdl2Keycode::End => Key::End,
        Sdl2Keycode::Home => Key::Home,
        Sdl2Keycode::Left => Key::Left,
        Sdl2Keycode::Up => Key::Up,
        Sdl2Keycode::Right => Key::Right,
        Sdl2Keycode::Down => Key::Down,
        Sdl2Keycode::Insert => Key::Insert,
        Sdl2Keycode::Delete => Key::Delete,
        Sdl2Keycode::Num0 => Key::Num0,
        Sdl2Keycode::Num1 => Key::Num1,
        Sdl2Keycode::Num2 => Key::Num2,
        Sdl2Keycode::Num3 => Key::Num3,
        Sdl2Keycode::Num4 => Key::Num4,
        Sdl2Keycode::Num5 => Key::Num5,
        Sdl2Keycode::Num6 => Key::Num6,
        Sdl2Keycode::Num7 => Key::Num7,
        Sdl2Keycode::Num8 => Key::Num8,
        Sdl2Keycode::Num9 => Key::Num9,
        Sdl2Keycode::A => Key::A,
        Sdl2Keycode::B => Key::B,
        Sdl2Keycode::C => Key::C,
        Sdl2Keycode::D => Key::D,
        Sdl2Keycode::E => Key::E,
        Sdl2Keycode::F => Key::F,
        Sdl2Keycode::G => Key::G,
        Sdl2Keycode::H => Key::H,
        Sdl2Keycode::I => Key::I,
        Sdl2Keycode::J => Key::J,
        Sdl2Keycode::K => Key::K,
        Sdl2Keycode::L => Key::L,
        Sdl2Keycode::M => Key::M,
        Sdl2Keycode::N => Key::N,
        Sdl2Keycode::O => Key::O,
        Sdl2Keycode::P => Key::P,
        Sdl2Keycode::Q => Key::Q,
        Sdl2Keycode::R => Key::R,
        Sdl2Keycode::S => Key::S,
        Sdl2Keycode::T => Key::T,
        Sdl2Keycode::U => Key::U,
        Sdl2Keycode::V => Key::V,
        Sdl2Keycode::W => Key::W,
        Sdl2Keycode::X => Key::X,
        Sdl2Keycode::Y => Key::Y,
        Sdl2Keycode::Z => Key::Z,
        Sdl2Keycode::Application => Key::Application,
        Sdl2Keycode::Select => Key::Select,
        Sdl2Keycode::Kp0 => Key::Kp0,
        Sdl2Keycode::Kp1 => Key::Kp1,
        Sdl2Keycode::Kp2 => Key::Kp2,
        Sdl2Keycode::Kp3 => Key::Kp3,
        Sdl2Keycode::Kp4 => Key::Kp4,
        Sdl2Keycode::Kp5 => Key::Kp5,
        Sdl2Keycode::Kp6 => Key::Kp6,
        Sdl2Keycode::Kp7 => Key::Kp7,
        Sdl2Keycode::Kp8 => Key::Kp8,
        Sdl2Keycode::Kp9 => Key::Kp9,
        Sdl2Keycode::KpMultiply => Key::KpMultiply,
        Sdl2Keycode::KpPlus => Key::KpPlus,
        Sdl2Keycode::KpMinus => Key::KpMinus,
        Sdl2Keycode::KpDecimal => Key::KpDecimal,
        Sdl2Keycode::KpDivide => Key::KpDivide,
        Sdl2Keycode::F1 => Key::F1,
        Sdl2Keycode::F2 => Key::F2,
        Sdl2Keycode::F3 => Key::F3,
        Sdl2Keycode::F4 => Key::F4,
        Sdl2Keycode::F5 => Key::F5,
        Sdl2Keycode::F6 => Key::F6,
        Sdl2Keycode::F7 => Key::F7,
        Sdl2Keycode::F8 => Key::F8,
        Sdl2Keycode::F9 => Key::F9,
        Sdl2Keycode::F10 => Key::F10,
        Sdl2Keycode::F11 => Key::F11,
        Sdl2Keycode::F12 => Key::F12,
        Sdl2Keycode::NumLockClear => Key::NumLockClear,
        Sdl2Keycode::ScrollLock => Key::ScrollLock,
        Sdl2Keycode::Semicolon => Key::Semicolon,
        Sdl2Keycode::Equals => Key::Equals,
        Sdl2Keycode::Comma => Key::Comma,
        Sdl2Keycode::Minus => Key::Minus,
        Sdl2Keycode::Period => Key::Period,
        Sdl2Keycode::Slash => Key::Slash,
        Sdl2Keycode::Backquote => Key::Backquote,
        Sdl2Keycode::LeftBracket => Key::LeftBracket,
        Sdl2Keycode::Backslash => Key::Backslash,
        Sdl2Keycode::RightBracket => Key::RightBracket,
        Sdl2Keycode::Quote => Key::Quote,
        _ => Key::Unknown,
    }
}
