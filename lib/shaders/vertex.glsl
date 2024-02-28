#version 140

uniform mat4 projection;

in vec3 position;
in vec4 color;

out vec4 vertex_color;

void main() {
    vertex_color = color;
    gl_Position = projection * vec4(position, 1.0);
}