#version 140

uniform sampler2D u_texture;
in vec2 v_texcoord;
out vec4 flagColor;
void main()
{
    float r = texture(u_texture, v_texcoord).r;
    flagColor = vec4(r,r,r,1);
}
