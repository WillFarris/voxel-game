#version 460

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in float block_type;

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
    
    vec4 pos = vec4(position, 1.0);
    //Block:  0
    //Grass:  1
    //Leaves: 2

    if(block_type == 0.0) {
        //pos.y += sin(time);
    }
    else if(block_type >= 0.0) {
        pos.x += 0.03 * sin(time) * cos(wind_speed * (time + 0.5 * pos.y));
        pos.z += 0.05 * cos(time) * sin(wind_speed * (time + 0.5 * pos.y));
        pos.y += 0.01 * sin(time + pos.x);
    }
    else {
        float m = mod(pos.y, 1.0);
        pos.x += m * 0.03 * sin(time) * cos(wind_speed * (time + 0.5 * pos.y));
        pos.z += m * 0.05 * cos(time) * sin(wind_speed * (time + 0.5 * pos.y));
        pos.y += m * 0.01 * sin(time + pos.x);

        //pos.x += mod(pos.y, 1.0) * sin(time + pos.y);
    }
    pos = camera_matrix * model_matrix * pos;

    v_position = pos.xyz;
    v_normal = transpose(inverse(mat3(model_matrix))) * normal;
    v_tex_coords = tex_coords;
    //v_block_type = block_type;

    gl_Position = pos;
}