#version 330 core

uniform usamplerBuffer data;
uniform uvec2 world_size;

in vec2 fs_tex_coords;
out vec4 color;

vec4 material_colors[3] = vec4[3] (
    vec4(0.0),                          // Air
    vec4(0.702, 0.4941, 0.0863, 1.0),   // Sand
    vec4(0.1922, 0.1529, 0.7608, 1.0)   // Water
);

vec3 material_color_variance[3] = vec3[3] (
    vec3(0.0),                          // Air
    vec3(0.07, 0.05, 0.02),             // Sand
    vec3(0.01)                          // Water
);

void main() {
    ivec2 screenpos = ivec2(fs_tex_coords);
    uvec4 d = texelFetch(data, (screenpos.y * int(world_size.x)) + screenpos.x);
    float seed = ((float(d.y) - 128.0) / 128.0); // Between 1.0 and -1.0 (aprox)

    vec4 base_color = material_colors[d.x];
    vec4 variance = vec4(seed * material_color_variance[d.x], 1.0);
    vec4 final_color = base_color + variance;

    color = final_color;
}