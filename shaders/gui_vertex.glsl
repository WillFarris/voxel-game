#version 150

#ifdef GL_ES
precision mediump float;
#endif

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    v_position = position;
    v_normal = normal;
    v_tex_coords = tex_coords;

    gl_Position = vec4(position.xyz, 1.0);
}