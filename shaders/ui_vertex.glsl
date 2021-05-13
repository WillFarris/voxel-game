#version 140

in vec2 position;

uniform vec2 u_dimensions;

void main() {
    float aspect = u_dimensions.y/u_dimensions.x;

    vec2 pos = position;
    pos.x *= aspect;
    
    gl_Position = vec4(pos, 0.0, 1.0);
}