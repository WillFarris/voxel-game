#version 460

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform float selected;
uniform sampler2D texture_map;

out vec4 color;

void main() {
    vec4 tex_color = texture(texture_map, v_tex_coords).rgba;

    float normalized_position_x = floor((v_position.x + 0.5) * 10.0);
    if(normalized_position_x == selected) {// - 0.001 && normalized_position_x < selected + 0.001) {
        tex_color.rgba += 0.4;
    }

    color = tex_color;
}