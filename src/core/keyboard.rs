extern crate glfw;

struct Keyboard {
    keys: [bool; 350]
}

static mut _KB: Keyboard = Keyboard { keys: [false; 350] };

pub fn is_pressed(key: glfw::Key) -> bool {
    unsafe { _KB.keys[std::mem::transmute::<glfw::Key, i32>(key) as usize] }
}

pub fn is_pressed_unchecked(key: usize) -> bool {
    unsafe { _KB.keys[key] }
}

pub fn key_event(key: glfw::Key, action: glfw::Action) {
    match action {
        glfw::Action::Press => unsafe {
            let code: i32 = std::mem::transmute::<glfw::Key, i32>(key);
            _KB.keys[code as usize] = true;
        }
        glfw::Action::Release => unsafe {
            let code: i32 = std::mem::transmute::<glfw::Key, i32>(key);
            _KB.keys[code as usize] = false;
        }
        _ => {},
    };
}

