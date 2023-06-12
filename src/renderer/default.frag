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
    /*
    if (fTexID != 0.0)
    {
        color = fColor * texture(uTextures[int(fTexID - 1)], fTexUV);
    }
    */
    if (fTexID == 1.0)
    {
        color = fColor * texture(uTextures[0], fTexUV);
    }
    else if (fTexID == 2.0)
    {
        color = fColor * texture(uTextures[1], fTexUV);
    }
    else if (fTexID == 3.0)
    {
        color = fColor * texture(uTextures[2], fTexUV);
    }
    else if (fTexID == 4.0)
    {
        color = fColor * texture(uTextures[3], fTexUV);
    }
    else if (fTexID == 5.0)
    {
        color = fColor * texture(uTextures[4], fTexUV);
    }
    else if (fTexID == 6.0)
    {
        color = fColor * texture(uTextures[5], fTexUV);
    }
    else if (fTexID == 7.0)
    {
        color = fColor * texture(uTextures[6], fTexUV);
    }
    else if (fTexID == 8.0)
    {
        color = fColor * texture(uTextures[7], fTexUV);
    }
    else
    {
        color = fColor;
    }
}
