#version 330 core

in vec4 fPos;
in vec4 fColor;
// in vec2 fTexCoords;
// in float fTexID;

// uniform sampler2D uTextures[8];
// uniform float uTime;

out vec4 color;

void main()
{
    color = fColor;
}

