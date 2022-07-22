extern crate glfw;

struct Mouse {
    buttons: [bool; 6],
    scroll_x: f64,
    scroll_y: f64,
    pos_x: f64,
    pos_y: f64,
    dpos_x: f64,
    dpos_y: f64,
}

static mut _MO: Mouse = Mouse {
    buttons: [false; 6],
    scroll_x: 0.0,
    scroll_y: 0.0,
    pos_x: 0.0,
    pos_y: 0.0,
    dpos_x: 0.0,
    dpos_y: 0.0,
};

pub fn is_pressed(button: glfw::MouseButton) -> bool {
    unsafe { _MO.buttons[std::mem::transmute::<glfw::MouseButton, i32>(button) as usize] }
}

pub fn is_pressed_unchecked(button: usize) -> bool {
    unsafe { _MO.buttons[button] }
}

pub fn pos() -> (f64, f64) {
    unsafe { (_MO.pos_x, _MO.pos_y) }
}

pub fn dpos() -> (f64, f64) {
    unsafe { (_MO.dpos_x, _MO.dpos_y) }
}

pub fn scroll_x() -> f64 {
    unsafe { _MO.scroll_x }
}

pub fn scroll_y() -> f64 {
    unsafe { _MO.scroll_y }
}

pub fn mouse_button_event(button: glfw::MouseButton, action: glfw::Action) {
    match action {
        glfw::Action::Press => unsafe {
            let code: i32 = std::mem::transmute::<glfw::MouseButton, i32>(button);
            _MO.buttons[code as usize] = true;
        }
        glfw::Action::Release => unsafe {
            let code: i32 = std::mem::transmute::<glfw::MouseButton, i32>(button);
            _MO.buttons[code as usize] = false;
        }
        _ => {}
    }
}

pub fn mouse_pos_event(x: f64, y: f64) {
    unsafe {
        _MO.dpos_x = x - _MO.pos_x;
        _MO.dpos_y = y - _MO.pos_y;
        _MO.pos_x = x;
        _MO.pos_y = y;
    }
}

pub fn mouse_scroll_event(x: f64, y: f64) {
    unsafe {
        _MO.scroll_x = x;
        _MO.scroll_y = y;
    }
}

