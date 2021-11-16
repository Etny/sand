#version 330 core

uniform sampler2D data;
in vec2 fs_tex_coords;

out vec4 color;

void main() {
    color = texture(data, fs_tex_coords);
}