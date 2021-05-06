#version 150

in vec3 position;
in vec3 normal;

uniform mat4 matrix;
uniform mat4 perspective;

//out vec2 v_tex_coords;
out vec3 v_normal;

void main() {
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = perspective * matrix * vec4(position, 1.0);
    //v_tex_coords = tex_coords;
}