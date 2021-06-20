#version 150

in vec3 v_position;
in vec2 v_tex_coords;

uniform sampler2D texture_map;

out vec4 color;
out vec3 normal;

void main() {
    vec4 tex_color = texture(texture_map, v_tex_coords).rgba;
    //if(tex_color.a < 0.5) { discard; }
    color = tex_color;
    //color = vec4(1.0, 0.0, 0.0, 1.0);
}