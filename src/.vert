#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 UV;

uniform vec3 camreAngles;
out vec2 fUV;

void main()
{
    gl_Position = vec4(Position, 0,1);
    fUV = UV;
}