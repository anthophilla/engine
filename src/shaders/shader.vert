#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec4 vertexColor;
out vec2 texCoord;

uniform vec3 offset;
uniform mat4 transform;

void main() {
    vec3 pos = aPos.xyz + offset.xyz;
    gl_Position = transform * vec4(pos, 1.0);
    vertexColor = aColor;
    texCoord = aTexCoord;
}