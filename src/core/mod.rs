pub mod world;
pub mod config;
pub mod mouse;
pub mod keyboard;
pub mod scene;
pub mod state;
pub mod window;

pub mod prelude {
    pub use crate::core::config::Config;
    pub use crate::core::scene::Scene;
    pub use crate::core::state::{State, StateError, StateMachine};
    pub use crate::core::world::{World, RageResult};

    pub mod mouse {
        pub use crate::core::mouse::{
            is_pressed,
            pos,
            dpos,
            scroll_x,
            scroll_y,
        };
    }

    pub mod keyboard {
        pub use crate::core::keyboard::is_pressed;
    }

    pub mod window {
        pub use crate::core::window::{
            get_width_height,
            set_width_height,
            get_title,
            set_title,
        };
    }
}

