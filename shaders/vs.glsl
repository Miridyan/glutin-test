#version 140

in vec2 position;
out vec4 my_attr;

uniform mat4 perspective;
uniform mat4 matrix;

void main() {
    my_attr = matrix * vec4(position, 0.0, 1.0);
    gl_Position = my_attr;
}
