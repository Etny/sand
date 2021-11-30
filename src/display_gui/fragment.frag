#version 330 core

uniform usamplerBuffer data;
uniform uvec2 world_size;

in vec2 fs_tex_coords;
out vec4 color;

vec4 material_colors[6] = vec4[6] (
    vec4(0.0),                          // Air
    vec4(0.702, 0.4941, 0.0863, 1.0),   // Sand
    vec4(0.1529, 0.3451, 0.7608, 1.0),  // Water
    vec4(0.2392, 0.098, 0.0039, 1.0),   // Wood
    vec4(0.9137, 0.35, 0.0627, 1.0),    // Fire
    vec4(0.9137, 0.35, 0.0627, 1.0)     // Flames

);

vec3 material_color_variance[6] = vec3[6] (
    vec3(0.0),                          // Air
    vec3(0.07, 0.05, 0.02),             // Sand
    vec3(0.01),                         // Water
    vec3(0.04),                         // Wood
    vec3(0.0, 0.25, 0.0),               // Fire
    vec3(0.0, 0.25, 0.0)                // Flames
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