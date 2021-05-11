#version 150

in vec3 v_normal;
in vec3 v_normal_model;
in vec3 v_position;
in vec3 v_color;
in vec2 v_tex_coords;

uniform vec3 u_color;
uniform vec3 light;
uniform vec3 u_position;
uniform vec3 u_direction;

uniform sampler2D tex;

out vec4 color;
out vec3 normal;

void main() {
    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.05);


    vec3 tex_color = texture(tex, v_tex_coords).rgb;

    vec3 camera_dir = normalize(-v_position);
    //vec3 half_direction = normalize(normalize(light) + camera_dir);
    //float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(vec3(0.01) + diffuse * tex_color, 1.0); //+ specular * vec3(0.9), 1.0);
}