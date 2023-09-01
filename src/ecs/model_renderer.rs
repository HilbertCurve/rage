use std::mem::size_of;

use glam::Vec3;
use glam::Vec4;
use json_rs::json::Cast;

use crate::prelude::Block;
use crate::prelude::Component;
use crate::prelude::ComponentError;
use crate::prelude::DynComponent;
use crate::prelude::Texture;
use crate::prelude::Model;
use crate::renderer::buffer::VProp;
use crate::renderer::buffer::VertexBuffer;
use crate::renderer::model::MODEL_VB;
use crate::renderer::renderer::RenderError;
use crate::renderer::renderer::Renderable;

#[derive(Component)]
pub struct ModelRenderer {
    back: Model,
    tex: Texture,
}

impl ModelRenderer {
    pub fn from(m: Model) -> ModelRenderer {
        ModelRenderer {
            back: m,
            tex: Texture { id: 0, uvs: [0.0;8] }
        }
    }
}

impl DynComponent for ModelRenderer {
    unsafe fn start(&mut self, _parent: *mut crate::prelude::Entity) -> Result<(), crate::prelude::ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, _parent: *mut crate::prelude::Entity) -> Result<(), crate::prelude::ComponentError> {
        self.to_buffer(&mut MODEL_VB)
            .map_err(|err| ComponentError::BadUpdate(format!("Error rendering ModelRenderer: {}", err.to_string())))
    }
    unsafe fn stop(&mut self, _parent: *mut crate::prelude::Entity) -> Result<(), ComponentError> {
        Ok(())
    }
}

impl Renderable for ModelRenderer {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError> {
        match &self.back {
            Model::GLTF { src: _, json, bin } => {
                // read buffers using glTF metadata
                let accessors = &json["accessors"];
                let views = &json["bufferViews"];
                let mesh_prim = &json["meshes"][0]["primitives"][0];

                let i_indices: usize = mesh_prim["indices"].cast().unwrap();
                let a_indices: usize = accessors[i_indices]["bufferView"].cast().unwrap();
                let b_indices = &views[a_indices];

                let (len_indices, off_indices): (usize, usize) = (
                    b_indices["byteLength"].cast().unwrap(),
                    b_indices["byteOffset"].cast().unwrap(),
                );

                let indices = Block::from(&bin.to_vec()[off_indices..len_indices+off_indices]);

                let i_pos: usize = mesh_prim["attributes"]["POSITION"].cast().unwrap();
                let a_pos: usize = accessors[i_pos]["bufferView"].cast().unwrap();
                let b_pos = &views[a_pos];

                let (len_pos, off_pos): (usize, usize) = (
                    b_pos["byteLength"].cast().unwrap(),
                    b_pos["byteOffset"].cast().unwrap(),
                );

                let pos_buf = Block::from(&bin.to_vec()[off_pos..len_pos+off_pos]);

                let i_norm: usize = mesh_prim["attributes"]["NORMAL"].cast().unwrap();
                let a_norm: usize = accessors[i_norm]["bufferView"].cast().unwrap();
                let b_norm = &views[a_norm];

                let (len_norm, off_norm): (usize, usize) = (
                    b_norm["byteLength"].cast().unwrap(),
                    b_norm["byteOffset"].cast().unwrap(),
                );

                let norm_buf = Block::from(&bin.to_vec()[off_norm..len_norm+off_norm]);

                buf.ib = indices;

                if pos_buf.len() != norm_buf.len() {
                    return Err(
                        RenderError::from(
                            &format!("buffer length mismatch: pos: {}, norm: {}", pos_buf.len(), norm_buf.len())
                        )
                    );
                }

                // temporary assertion: assume position and normal values are Vec3's
                // TODO: accomodate for variable vector types
                {
                    let t: String = accessors[i_pos]["type"].cast().unwrap();
                    assert_eq!("VEC3", t);
                    let t: String = accessors[i_pos]["type"].cast().unwrap();
                    assert_eq!("VEC3", t);
                }

                // buffer insertion

                // TODO: this assumes that the buffer has the proper vertex sizes. Either add a check or an accomodation for this
                let (pos_pos, _, pos_type_enum) = buf.attrib_metadata(VProp::Position)?;
                let (col_pos, _, col_type_enum) = buf.attrib_metadata(VProp::Color)?;
                let (norm_pos, _, norm_type_enum) = buf.attrib_metadata(VProp::Other)?;
                let (tuv_pos, _, tuv_type_enum) = buf.attrib_metadata(VProp::TexUV)?;
                let (tid_pos, _, tid_type_enum) = buf.attrib_metadata(VProp::TexID)?;

                let mut offset = buf.vb.len();

                // loop over every Vec3 in the glTF vertex buffer
                for i in 0..accessors[i_pos]["count"].cast().unwrap() {
                    // prepare a new vertex to push
                    for _ in 0..buf.layout_len() {
                        buf.vb.push(0u8);
                    }

                    // insert position data into vertex
                    for j in 0..3 {
                        // don't blame me, C++ made me crazy

                        // vertex * size of position vector + [x, y, or z] * size of float
                        let pos_value = unsafe { pos_buf.get::<f32>(
                            i * size_of::<Vec3>() + j * size_of::<f32>()
                        ).unwrap() };
                        buf.vb.set(
                            offset + (pos_pos + j) * pos_type_enum.size_bytes(),
                            pos_value)
                            .or(Err(RenderError::from(&format!("bad block insertion"))))?;
                    }
                    // for the time being, insert the color white
                    for j in 0..4 {
                        buf.vb.set(
                            offset + (col_pos + j) * col_type_enum.size_bytes(),
                            Vec4::ONE)
                            .or(Err(RenderError::from(&format!("bad block insertion"))))?;
                    }
                    // insert normal data into vertex
                    for j in 0..3 {
                        // if it doesn't you ain't doin' it right

                        let norm_value = unsafe { norm_buf.get::<f32>(
                            i * size_of::<Vec3>() + j*size_of::<f32>()
                        ).unwrap() };
                        buf.vb.set(
                            offset + (norm_pos + j) * norm_type_enum.size_bytes(),
                            norm_value)
                            .or(Err(RenderError::from(&format!("bad block insertion"))))?;
                    }

                    //TODO: model textures
                    for i in 0..2 {
                        buf.vb.set(
                            offset + (tuv_pos + i) * tuv_type_enum.size_bytes(),
                            0.0)
                            .or(Err(RenderError::from(&format!("bad block insertion"))))?;
                    }
                    buf.vb.set(
                        offset + tid_pos * tid_type_enum.size_bytes(),
                        self.tex.id as f32)
                        .or(Err(RenderError::from(&format!("bad block insertion"))))?;

                    offset += buf.layout_len();
                }
                println!("{}", buf.vb.len())
            }
            Model::Obj { src: _, vals: _ } => {
                todo!("")
            }
        }

        Ok(())
    }
}
