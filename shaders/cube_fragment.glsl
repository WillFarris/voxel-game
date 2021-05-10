#version 140

in vec3 v_normal;
in vec3 v_normal_model;
in vec3 v_position;
in vec3 v_color;

uniform vec3 u_color;
uniform vec3 light;
uniform vec3 u_position;
uniform vec3 u_direction;

out vec4 color;
out vec3 normal;

void main() {
    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(vec3(0.0) + diffuse * u_color + specular * vec3(1.0), 1.0);
}