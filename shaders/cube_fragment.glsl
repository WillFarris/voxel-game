#version 150

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

//uniform vec3 light;

//uniform sampler2D tex;

out vec4 color;
out vec3 normal;

void main() {

    vec3 light = vec3(0.701, 0.701, -0.701);


    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.05);


    vec3 tex_color = vec3(0.1, 0.9, 0.1);//texture(tex, v_tex_coords).rgb;

    vec3 camera_dir = normalize(-v_position);
    //vec3 half_direction = normalize(normalize(light) + camera_dir);
    //float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    //tex_color = tex_color + specular * vec3(0.9);

    color = vec4(vec3(0.01) + diffuse * tex_color, 1.0);
}