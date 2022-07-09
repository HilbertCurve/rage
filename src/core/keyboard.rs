extern crate glfw;

struct Keyboard {
    keys: [bool; 350]
}

static mut _KB: Keyboard = Keyboard { keys: [false; 350] };

impl Keyboard {
    pub fn is_pressed_unchecked(key: usize) -> bool {
        unsafe { _KB.keys[key] }
    }

    pub fn register_key_press(key: glfw::Key) {
        unsafe {
            let code: i32 = std::mem::transmute::<glfw::Key, i32>(key);
            _KB.keys[code as usize] = true;
        }
    }
    
    pub fn register_key_release(key: glfw::Key) {
        unsafe {
            let code: i32 = std::mem::transmute::<glfw::Key, i32>(key);
            _KB.keys[code as usize] = false;
        }
    }
}

pub fn is_pressed(key: glfw::Key) -> bool {
    let ikey = unsafe { std::mem::transmute::<glfw::Key, i32>(key) };

    Keyboard::is_pressed_unchecked(ikey as usize)
}

pub fn key_event(key: glfw::Key, action: glfw::Action) {
    match action {
        glfw::Action::Press => Keyboard::register_key_press(key),
        glfw::Action::Release => Keyboard::register_key_release(key),
        _ => {},
    };
}

