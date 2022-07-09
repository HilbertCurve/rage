pub struct Mouse {
    buttons: [bool; 6],
    scroll_x: i32,
    scroll_y: i32,
    pos_x: i32,
    pos_y: i32,
    dpos_x: i32,
    dpos_y: i32,
}

static mut _MO: Mouse = Mouse {
    buttons: [false; 6],
    scroll_x: 0,
    scroll_y: 0,
    pos_x: 0,
    pos_y: 0,
    dpos_x: 0,
    dpos_y: 0,
};

impl Mouse {
    pub fn is_pressed(button: usize) -> Result<bool, String> {
        let buttons = unsafe { &_MO.buttons };
        if button > buttons.len() {
            Err(format!("Invalid button: {}", button))
        } else {
            Ok(buttons[button])
        }
    }

    pub fn is_pressed_unchecked(button: usize) -> bool {
        unsafe { _MO.buttons[button] }
    }

    pub fn pos() -> (i32, i32) {
        unsafe { (_MO.pos_x, _MO.pos_y) }
    }

    pub fn dpos() -> (i32, i32) {
        unsafe { (_MO.dpos_x, _MO.dpos_y) }
    }

    pub fn scroll_x() -> i32 {
        unsafe { _MO.scroll_x }
    }

    pub fn scroll_y() -> i32 {
        unsafe { _MO.scroll_y }
    }

}

