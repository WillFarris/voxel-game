#version 140

uniform vec3 pot_color;
in vec3 v_normal;
out vec4 color;
uniform vec3 light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(light));
    vec3 black = vec3(0.1, 0.0, 0.0);
    color = vec4(mix(black, pot_color, brightness), 1.0);
    //color = vec4(v_normal, 1.0);
}