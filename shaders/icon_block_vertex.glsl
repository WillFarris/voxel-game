#version 460

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_position;
out vec2 v_tex_coords;

void main() {

    v_position = position.xyz;
    v_tex_coords = tex_coords;

    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}