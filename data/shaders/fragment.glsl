#version 150 core

in vec2 f_uv;
out vec4 out_color;

uniform sampler2D sampler;

void main() {
    out_color = texture(sampler, f_uv);
}
