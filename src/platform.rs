use std::time::Instant;

use imgui::{Context, Io, MouseButton};
use sfml::window::{Event, mouse::{Wheel, Button}, Key};

fn handle_key(io: &mut Io, key: Key, pressed: bool) {
    let igkey = match key {
        Key::A => imgui::Key::A,
        Key::B => imgui::Key::B,
        Key::C => imgui::Key::C,
        Key::D => imgui::Key::D,
        Key::E => imgui::Key::E,
        Key::F => imgui::Key::F,
        Key::G => imgui::Key::G,
        Key::H => imgui::Key::H,
        Key::I => imgui::Key::I,
        Key::J => imgui::Key::J,
        Key::K => imgui::Key::K,
        Key::L => imgui::Key::L,
        Key::M => imgui::Key::M,
        Key::N => imgui::Key::N,
        Key::O => imgui::Key::O,
        Key::P => imgui::Key::P,
        Key::Q => imgui::Key::Q,
        Key::R => imgui::Key::R,
        Key::S => imgui::Key::S,
        Key::T => imgui::Key::T,
        Key::U => imgui::Key::U,
        Key::V => imgui::Key::V,
        Key::W => imgui::Key::W,
        Key::X => imgui::Key::X,
        Key::Y => imgui::Key::Y,
        Key::Z => imgui::Key::Z,
        Key::Num1 => imgui::Key::Keypad1,
        Key::Num2 => imgui::Key::Keypad2,
        Key::Num3 => imgui::Key::Keypad3,
        Key::Num4 => imgui::Key::Keypad4,
        Key::Num5 => imgui::Key::Keypad5,
        Key::Num6 => imgui::Key::Keypad6,
        Key::Num7 => imgui::Key::Keypad7,
        Key::Num8 => imgui::Key::Keypad8,
        Key::Num9 => imgui::Key::Keypad9,
        Key::Num0 => imgui::Key::Keypad0,
        Key::Enter => imgui::Key::Enter, // TODO: Should this be treated as alias?
        Key::Escape => imgui::Key::Escape,
        Key::Backspace => imgui::Key::Backspace,
        Key::Tab => imgui::Key::Tab,
        Key::Space => imgui::Key::Space,
        Key::Subtract => imgui::Key::Minus,
        Key::Equal => imgui::Key::Equal,
        Key::LBracket => imgui::Key::LeftBracket,
        Key::RBracket => imgui::Key::RightBracket,
        Key::Backslash => imgui::Key::Backslash,
        Key::Semicolon => imgui::Key::Semicolon,
        Key::Quote => imgui::Key::Apostrophe,
        Key::Tilde => imgui::Key::GraveAccent,
        Key::Comma => imgui::Key::Comma,
        Key::Period => imgui::Key::Period,
        Key::Slash => imgui::Key::Slash,
        // Key::CapsLock => imgui::Key::CapsLock,
        Key::F1 => imgui::Key::F1,
        Key::F2 => imgui::Key::F2,
        Key::F3 => imgui::Key::F3,
        Key::F4 => imgui::Key::F4,
        Key::F5 => imgui::Key::F5,
        Key::F6 => imgui::Key::F6,
        Key::F7 => imgui::Key::F7,
        Key::F8 => imgui::Key::F8,
        Key::F9 => imgui::Key::F9,
        Key::F10 => imgui::Key::F10,
        Key::F11 => imgui::Key::F11,
        Key::F12 => imgui::Key::F12,
        // Key::PrintScreen => imgui::Key::PrintScreen,
        // Key::ScrollLock => imgui::Key::ScrollLock,
        Key::Pause => imgui::Key::Pause,
        Key::Insert => imgui::Key::Insert,
        Key::Home => imgui::Key::Home,
        Key::PageUp => imgui::Key::PageUp,
        Key::Delete => imgui::Key::Delete,
        Key::End => imgui::Key::End,
        Key::PageDown => imgui::Key::PageDown,
        Key::Right => imgui::Key::RightArrow,
        Key::Left => imgui::Key::LeftArrow,
        Key::Down => imgui::Key::DownArrow,
        Key::Up => imgui::Key::UpArrow,
        // Key::KpDivide => imgui::Key::KeypadDivide,
        // Key::KpMultiply => imgui::Key::KeypadMultiply,
        // Key::KpMinus => imgui::Key::KeypadSubtract,
        // Key::KpPlus => imgui::Key::KeypadAdd,
        // Key::KpEnter => imgui::Key::KeypadEnter,
        Key::Numpad1 => imgui::Key::Keypad1,
        Key::Numpad2 => imgui::Key::Keypad2,
        Key::Numpad3 => imgui::Key::Keypad3,
        Key::Numpad4 => imgui::Key::Keypad4,
        Key::Numpad5 => imgui::Key::Keypad5,
        Key::Numpad6 => imgui::Key::Keypad6,
        Key::Numpad7 => imgui::Key::Keypad7,
        Key::Numpad8 => imgui::Key::Keypad8,
        Key::Numpad9 => imgui::Key::Keypad9,
        Key::Numpad0 => imgui::Key::Keypad0,
        // Key::KpPeriod => imgui::Key::KeypadDecimal,
        // Key::KpEquals => imgui::Key::KeypadEqual,
        Key::Menu => imgui::Key::Menu,
        Key::LControl => imgui::Key::LeftCtrl,
        Key::LShift => imgui::Key::LeftShift,
        Key::LAlt => imgui::Key::LeftAlt,
        Key::LSystem => imgui::Key::LeftSuper,
        Key::RControl => imgui::Key::RightCtrl,
        Key::RShift => imgui::Key::RightShift,
        Key::RAlt => imgui::Key::RightAlt,
        Key::RSystem => imgui::Key::RightSuper,
        _ => {
            // Ignore unknown keys
            return;
        }
    };

    io.add_key_event(igkey, pressed);
}


pub struct SFMLPlatform {
    last_frame: Instant,
}

impl SFMLPlatform {
    pub fn init(imgui: &mut Context) -> Self {
        // TODO: set flags

        imgui.set_platform_name(Some(format!(
            "imgui-sfml-support {}",
            env!("CARGO_PKG_VERSION")
        )));

        Self {
            last_frame : Instant::now(),
        }
    }

    pub fn handle_event(&self, imgui: &mut Context, event: Event) {
        let mut io = imgui.io_mut();
        match event {
            Event::MouseMoved { x, y } => {
                io.mouse_pos = [ x as f32, y as f32 ];
            },
            Event::MouseWheelScrolled { wheel, delta, .. } => {
                let param = match wheel {
                    Wheel::VerticalWheel => [0f32, delta],
                    Wheel::HorizontalWheel => [delta, 0f32],
                };

                io.add_mouse_wheel_event(param)
            },
            Event::MouseButtonPressed { button, .. }
            | Event::MouseButtonReleased { button, .. }=> {
                let imgui_button = match button {
                    Button::Left => MouseButton::Left,
                    Button::Right => MouseButton::Right,
                    Button::Middle => MouseButton::Middle,
                    Button::XButton1 => MouseButton::Extra1,
                    Button::XButton2 => MouseButton::Extra2,
                };

                let down = if let Event::MouseButtonPressed { .. } = event { true } else { false };
                io.add_mouse_button_event(imgui_button, down);
            },
            Event::TextEntered { unicode } => {
                io.add_input_character(unicode);
            },
            Event::KeyPressed { code, alt, ctrl, shift, system, .. }
            | Event::KeyReleased { code, alt, ctrl, shift, system, .. } => {
                use imgui::Key as K;
                io.add_key_event(K::ModShift, shift);
                io.add_key_event(K::ModCtrl, ctrl);
                io.add_key_event(K::ModAlt, alt);
                io.add_key_event(K::ModSuper, system);
                let pressed = if let Event::KeyPressed { .. } = event { true } else { false };
                handle_key(&mut io, code, pressed);
            },
            _ => (),
        }
    }

    pub fn prepare_frame(&mut self, imgui: &mut Context) {
        let io = imgui.io_mut();

        let now = Instant::now();
        io.update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;

        // TODO: resize display & framebuffer scale
        // io.display_size
        // io.display_framebuffer_scale

        // TODO:
        // if io.want_set_mouse_pos {
            // move mouse to requested coordinates
        // }

        // TODO: update mouse icon
    }
}
