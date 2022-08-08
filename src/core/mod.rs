pub mod app;
pub mod config;
pub mod mouse;
pub mod keyboard;

pub mod prelude {
    pub mod app {
        pub use crate::core::app::run;
    }
    pub use super::config::Config;
    pub mod mouse {
        pub use crate::core::mouse::{is_pressed, pos, dpos, scroll_x, scroll_y};
    }
    pub mod keyboard {
        pub use crate::core::keyboard::is_pressed;
    }
}

