#version 460

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;
uniform float time;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {
    float wind_speed = 5.0;
    mat4 camera_matrix = perspective_matrix * view_matrix;
    
    vec4 pos = vec4(position.xyz, 1.0);

    pos.xz += mod(position.y, 1.0) * 0.03 * sin(wind_speed * (time + pos.y));
    
    pos = camera_matrix * model_matrix * pos;
    
    
    v_position = pos.xyz;
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_tex_coords = tex_coords;

    gl_Position = pos;
}