#version 150

in vec3 position;
in vec3 normal;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

//out vec2 v_tex_coords;
out vec3 v_normal;

void main() {
    v_normal = transpose(inverse(mat3(view_matrix))) * normal;
    gl_Position = perspective_matrix * view_matrix * model_matrix * vec4(position, 1.0);
}