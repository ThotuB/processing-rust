#version 140

#ifdef GL_ES
precision mediump float;
precision mediump int;
#endif

in vec4 vertex_color;

void main() {
    gl_FragColor = vertex_color;
}