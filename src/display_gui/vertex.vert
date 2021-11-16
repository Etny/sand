#version 330 core

layout (location = 0) in uvec2 position;
layout (location = 1) in vec4 color;

out vec4 color_fs;
uniform uvec2 dimensions;
uniform float point_size;


void main() {

    color_fs = color;

    float point_size_ss = point_size / dimensions.x;

    vec2 real_pos = vec2(((position.x / (dimensions.x - 1.0)) * 2.0) - 1.0, ((position.y / (dimensions.y - 1.0)) * 2.0) - 1.0);
    vec2 correct = vec2(real_pos.x - point_size_ss / 2, real_pos.y - point_size_ss / 2);
    gl_Position = vec4(correct, 0, 1);
}