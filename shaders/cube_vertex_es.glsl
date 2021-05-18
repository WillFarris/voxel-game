#version 310 es

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;
    
    
    gl_Position = camera_matrix * model_matrix * vec4(position, 1.0);

    v_position =  (camera_matrix * vec4(position, 1.0)).xyz;
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_tex_coords = tex_coords;
    
}
