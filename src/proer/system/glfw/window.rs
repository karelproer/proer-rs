extern crate glfw;
use super::super::window;
use super::super::window::CursorMode;
use super::platform;
use super::super::event::{Event, Key};
use super::super::mousebutton::MouseButton;
use super::super::mousebutton::Action;
use glfw::Context;

pub struct Window {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

fn glfw_button_to_proer_button(button: glfw::MouseButton) -> MouseButton {
    match button {
        glfw::MouseButtonLeft => MouseButton::Left,
        glfw::MouseButtonRight => MouseButton::Right,
        glfw::MouseButtonMiddle => MouseButton::Middle,
        glfw::MouseButton::Button4 => MouseButton::X1,
        glfw::MouseButton::Button5 => MouseButton::X2,
        glfw::MouseButton::Button6 => MouseButton::X3,
        glfw::MouseButton::Button7 => MouseButton::X4,
        glfw::MouseButton::Button8 => MouseButton::X5,
    }
}

fn glfw_key_to_proer_key(key: glfw::Key) -> Option<Key> {
    use glfw::Key::*;
    match key {
        Up         => Some(Key::ArrowUp),
        Down       => Some(Key::ArrowDown),
        Right      => Some(Key::ArrowRight),
        Left       => Some(Key::ArrowLeft),
        Escape     => Some(Key::Escape),
        Tab        => Some(Key::Tab),
        Backspace  => Some(Key::Backspace),
        Enter      => Some(Key::Enter),
        Space      => Some(Key::Space),
        Insert     => Some(Key::Insert),
        Delete     => Some(Key::Delete),
        Home       => Some(Key::Home),
        End        => Some(Key::End),
        PageUp     => Some(Key::PageUp),
        PageDown   => Some(Key::PageDown),
        Minus      => Some(Key::Minus),
        Equal      => Some(Key::PlusEquals),
        Num0 | Kp0 => Some(Key::Num0),
        Num1 | Kp1 => Some(Key::Num1),
        Num2 | Kp2 => Some(Key::Num2),
        Num3 | Kp3 => Some(Key::Num3),
        Num4 | Kp4 => Some(Key::Num4),
        Num5 | Kp5 => Some(Key::Num5),
        Num6 | Kp6 => Some(Key::Num6),
        Num7 | Kp7 => Some(Key::Num7),
        Num8 | Kp8 => Some(Key::Num8),
        Num9 | Kp9 => Some(Key::Num9),
        A          => Some(Key::A),
        B          => Some(Key::B),
        C          => Some(Key::C),
        D          => Some(Key::D),
        E          => Some(Key::E),
        F          => Some(Key::F),
        G          => Some(Key::G),
        H          => Some(Key::H),
        I          => Some(Key::I),
        J          => Some(Key::J),
        K          => Some(Key::K),
        L          => Some(Key::L),
        M          => Some(Key::M),
        N          => Some(Key::N),
        O          => Some(Key::O),
        P          => Some(Key::P),
        Q          => Some(Key::Q),
        R          => Some(Key::R),
        S          => Some(Key::S),
        T          => Some(Key::T),
        U          => Some(Key::U),
        V          => Some(Key::V),
        W          => Some(Key::W),
        X          => Some(Key::X),
        Y          => Some(Key::Y),
        Z          => Some(Key::Z),
        F1         => Some(Key::F1),
        F2         => Some(Key::F2),
        F3         => Some(Key::F3),
        F4         => Some(Key::F4),
        F5         => Some(Key::F5),
        F6         => Some(Key::F6),
        F7         => Some(Key::F7),
        F8         => Some(Key::F8),
        F9         => Some(Key::F9),
        F10        => Some(Key::F10),
        F11        => Some(Key::F11),
        F13        => Some(Key::F13),
        F14        => Some(Key::F14),
        F15        => Some(Key::F15),
        F16        => Some(Key::F16),
        F17        => Some(Key::F17),
        F18        => Some(Key::F18),
        F19        => Some(Key::F19),
        F20        => Some(Key::F20),        

        _ => None
    }
}

impl window::Window for Window {
    type Platform = platform::Platform;

    fn new(platform: &mut Self::Platform, size: (u32, u32), title: &str) -> Self {
        let (mut window, events) = platform.get_glfw().create_window(size.0, size.1, title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
        log::info!("Created GLFW window.");
        window.set_all_polling(true);
        Self {
            window,
            events,
        }
    }

    fn open(&mut self, _platform: &mut Self::Platform) -> bool {
        !self.window.should_close()
    }

    fn update(&mut self, platform: &mut Self::Platform) {
        self.window.swap_buffers();
        platform.get_glfw().poll_events();
    }

    fn get_event(&mut self) -> Option<Event> {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Size(width, height) => return Some(Event::Resize((width.try_into().unwrap(), height.try_into().unwrap()))),
                glfw::WindowEvent::Close => return Some(Event::Close),
                glfw::WindowEvent::Pos(x, y) => return Some(Event::Move((x, y))),
                glfw::WindowEvent::Focus(true) => return Some(Event::Focus),
                glfw::WindowEvent::Focus(false) => return Some(Event::UnFocus),
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, _) => return Some(Event::Button(glfw_button_to_proer_button(button), Action::Press)),
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, _) => return Some(Event::Button(glfw_button_to_proer_button(button), Action::Release)),
                glfw::WindowEvent::MouseButton(button, glfw::Action::Repeat, _) => return Some(Event::Button(glfw_button_to_proer_button(button), Action::Repeat)),
                glfw::WindowEvent::CursorPos(x, y) => return Some(Event::CursorMove((x, y))),
                glfw::WindowEvent::Scroll(x, y) => return Some(Event::Scroll((x, y))),
                glfw::WindowEvent::Char(c) => return Some(Event::Char(c)),
                glfw::WindowEvent::Key(key, _, glfw::Action::Press, _)   => return Some(Event::Key(glfw_key_to_proer_key(key)?, Action::Press)),
                glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => return Some(Event::Key(glfw_key_to_proer_key(key)?, Action::Release)),
                glfw::WindowEvent::Key(key, _, glfw::Action::Repeat, _)  => return Some(Event::Key(glfw_key_to_proer_key(key)?, Action::Repeat)),
                _ => {},
            }
        }
        None
    }

    fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.window.set_cursor_mode(match mode {
            CursorMode::Normal   => glfw::CursorMode::Normal,
            CursorMode::Hidden   => glfw::CursorMode::Hidden,
            CursorMode::Disabled => glfw::CursorMode::Disabled,
        });
    }

    fn set_raw_mouse_input(&mut self, raw: bool, platform: &mut Self::Platform) -> bool {
        if platform.get_glfw().supports_raw_motion() {
            self.window.set_raw_mouse_motion(raw);
            true
        } else {false }
    }
}

impl window::OpenGLContext for Window {
    fn make_current(&mut self) {
        self.window.make_current();
    }

    fn get_proc_address(&mut self, procname: &str) -> *const std::os::raw::c_void {
        self.window.get_proc_address(procname) as *const std::os::raw::c_void
    }
}