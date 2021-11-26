#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 tex_coords;

out vec2 fs_tex_coords;


void main() {
    fs_tex_coords = tex_coords;
    gl_Position = vec4(position, 0, 1);
}