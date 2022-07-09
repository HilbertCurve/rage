pub mod core;
pub mod renderer;
pub mod utils;
// re-exports here

extern crate glam;
pub struct Rect {
//    pos: glam::Vec2,
}

//use renderer::{Renderable, RenderError, VertexBuffer};
/*
impl Renderable for Rect {
    fn to_buffer(buf: &mut VertexBuffer, pos: u32) -> Result<(), RenderError> {
        // get Position attribute
        let (pos_off, pos_len) = {
            let mut iter = buf.layout.iter();
            let mut acc = 0;
            let mut len = 0;
            loop {
                if let Some(attrib) = iter.next() {
                    let prop = attrib.v_prop;
                    if prop != renderer::VProp::Position {
                        acc += attrib.v_count as usize * attrib.v_type.size_bytes();
                    } else {
                        len = attrib.v_count;
                        break;
                    }
                } else {
                    return Err(RenderError::from("Vertex buffer has no position attribute, cannot insert data from Rect"))
                }
            }
            (acc, len)
        };
        
        // insert data
        if pos_len < 2 {
            Err(RenderError::from("Vertex buffer has position attribute that is too short, cannot insert data from Rect"))
        } else {
        }
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut _rect: Rect;
        let mut config: core::Config = core::Config::default();
        config.window_height = 600;
        core::run(config).expect("rage-quit");
    }

    #[test]
    fn block_test_push_pop() {
        let mut block: utils::Block = utils::Block::empty();
        block.push(4u32);

        let val: u32 = unsafe {
            block.pop::<u32>().expect("Pop failed")
        };

        assert_eq!(val, 4u32, "Popped value not expected value: expected 4, got {}", val);
        assert!(block.len() == 0, "Inconsistent block length: expected 0, got {}", block.len());

        block.push(2u32);
        block.push(1f32);

        assert!(block.len() == 8, "Inconsistend block length: expected 8, got {}", block.len());

        let (fval, uval) = unsafe {
            (
                block.pop::<f32>().expect("Pop failed"),
                block.pop::<u32>().expect("Pop failed"),
            )
        };

        assert_eq!(fval, 1.0, "Popped signed value not expected value: expected 4, got {}", fval);
        assert_eq!(uval, 2, "Popped unsigned value not expected value: expected 4, got {}", uval);
    }
}

