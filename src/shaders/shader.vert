#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec4 vertexColor;
out vec2 texCoord;

uniform mat4 perspective;
uniform mat4 view;

uniform vec3 model_trans;
uniform mat4 model_rot;

void main() {
    vec3 pos = aPos.xyz + model_trans.xyz;
    gl_Position = perspective * view * model_rot * vec4(pos, 1.0);
    //gl_Position = model_rot*vec4(pos, 1.0);
    vertexColor = aColor;
    texCoord = aTexCoord;
}