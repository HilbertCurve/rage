extern crate json_rs;

use json_rs::json::{JSONValue, Cast};

use crate::{renderer::{buffer::VertexBuffer, shader::Shader}, prelude::Block};

use std::{error::Error, path::{Path, PathBuf}};

pub static mut MODEL_VB: VertexBuffer = VertexBuffer::new();
pub static mut MODEL_SHADER: Shader = Shader::new_uninit();

pub const VERT_CODE: &'static str = 
"#version 330 core

layout (location=0) in vec3 aPos;
layout (location=1) in vec4 aColor;
layout (location=2) in vec3 aNorm;
layout (location=3) in vec2 aTexUV;
layout (location=4) in float aTexID;

uniform mat4 uProjection;
uniform mat4 uView;

out vec4 fPos;
out vec4 fColor;
out vec3 fNorm;
out vec2 fTexUV;
out float fTexID;

void main()
{
    fPos = vec4(aPos, 1.0);
    fColor = aColor;
    fNorm = aNorm;
    fTexUV = aTexUV;
    fTexID = aTexID;
    gl_Position = vec4(aPos, 1.0);
}";

pub const FRAG_CODE: &'static str =
"#version 330 core

in vec4 fPos;
in vec4 fColor;
in vec3 fNorm;
in vec2 fTexUV;
in float fTexID;

out vec4 color;

uniform sampler2D uTextures[8];

void main()
{
    color = vec4(1.0);
}";

type FIndex = (u64, u64, u64);

pub enum ObjValue {
    V (f64, f64, f64, f64),
    Vt (f64, f64, f64),
    Vn (f64, f64, f64),
    Vp (f64, f64, f64),
    F (FIndex, FIndex, FIndex),
    L (Vec<u64>),
}

pub enum Model {
    GLTF {
        src: String,
        json: JSONValue,
        bin: Block,
    },
    Obj {
        src: String,
        vals: Vec<ObjValue>,
    },
}

impl Model {
    pub fn from_gltf(src: &str) -> Result<Model, Box<dyn Error>> {
        let json = JSONValue::try_from(std::fs::read(src.clone())?)?;

        let bin_name: String = json["buffers"][0]["uri"].cast()?;

        let bin_path: PathBuf = Path::join(Path::new(src).parent().unwrap(), Path::new(&bin_name));

        Ok(Model::GLTF {
            src: src.to_owned(),
            json,
            bin: (Block::from_file(&mut std::fs::File::open(bin_path.to_str().unwrap())?)?),
        })
    }
}