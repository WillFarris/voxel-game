#version 310 es

precision mediump float;

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform vec3 sunlight_direction;
uniform sampler2D texture_map;

out vec4 color;

void main() {
    float diffuse = max(dot(normalize(v_normal), normalize(sunlight_direction)), 0.2);

    vec3 tex_color = texture(texture_map, v_tex_coords).rgb;

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(sunlight_direction) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    tex_color = tex_color + specular * vec3(0.9);

    color = vec4(vec3(0.01) + diffuse * tex_color, 1.0);
}
