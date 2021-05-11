#version 140

in vec2 tex_coords;

uniform sampler2D u_texture;

out vec4 color;

void main() {
    color = texture(u_texture, tex_coords);
}