// TODO: add more exhaustive traits for window (maximized, window pos, etc)
struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

static mut _WIN: Window = Window {    
    width: 0,
    height: 0,
    title: String::new(),
};

pub fn get_width_height() -> (u32, u32) {
    unsafe { (_WIN.width, _WIN.height) }
}
pub unsafe fn set_width_height(width: u32, height: u32) {
    _WIN.width = width;
    _WIN.height = height;
}
pub fn width_height_event(width: u32, height: u32) {
    unsafe {
        _WIN.width = width;
        _WIN.height = height;
    }
}

pub fn get_title() -> String {
    unsafe { _WIN.title.clone() }
}
pub unsafe fn set_title(title: String) {
    _WIN.title = title;
}
