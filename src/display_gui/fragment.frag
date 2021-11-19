#version 330 core

uniform usamplerBuffer data;
in vec2 fs_tex_coords;

out vec4 color;

void main() {
    vec2 uv = vec2(fs_tex_coords.x, 1 - fs_tex_coords.y);
    ivec2 screenpos = ivec2(int(uv.x * 640.0), int(uv.y * 320.0));
    uvec4 d = texelFetch(data, (screenpos.y * 640) + screenpos.x);
    float red = d.x == uint(1) ? 1.0 : 0.0;
    color = vec4(red, 0.0, 0.0, 1.0);
}