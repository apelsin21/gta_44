#version 150 core

in vec3 v_pos; 
in vec2 v_uv;

out vec2 f_uv;

uniform mat4 v_transform;

void main() {
    f_uv = v_uv;
    gl_Position = v_transform * vec4(v_pos, 1.0);
}
