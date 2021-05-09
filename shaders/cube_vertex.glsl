#version 150

in vec3 position;
in vec3 normal;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

//out vec2 v_tex_coords;
out vec3 v_normal;
out vec3 v_normal_model;
out vec3 v_position;

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;
    
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_normal_model = normal;
    v_position =  (camera_matrix * vec4(position, 1.0)).xyz;
    
    gl_Position = camera_matrix * model_matrix * vec4(position, 1.0);
}