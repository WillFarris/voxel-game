#version 150

in vec3 position;
in vec3 normal;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;
uniform vec3 u_position;
uniform vec3 u_direction;
uniform vec3 u_color;

out vec3 v_normal;
out vec3 v_normal_model;
out vec3 v_position;
out vec3 v_color;

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_normal_model = normal;
    v_position =  (camera_matrix * vec4(position, 1.0)).xyz;
    gl_Position = camera_matrix * model_matrix * vec4(position, 1.0);

    v_color = u_color * dot(u_position + u_direction, normal);
}