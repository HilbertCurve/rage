#version 330 core

in vec4 fPos;
in vec4 fColor;
in vec2 fTexUV;
in float fTexID;

uniform sampler2D uTextures[8];
// uniform float uTime;

out vec4 color;
void main()                          
{
    if (fTexID != 0.0)
    {
        color = fColor * texture(uTextures[int(fTexID - 1)], fTexUV);
    }
    else
    {
        color = fColor;
    }
}
