#version 430 core

layout (location = 0) in vec3 position;
//layout (location = 1) in vec4 color;
layout (location = 2) uniform mat4 transformation;
layout (location = 5) in vec3 normals;
//out vec4 newColor;
out vec3 newNormals;
//Task 4
//uniform mat4 transformation;

//mat4x4 matrix = mat4(1)

void main()
{
    gl_Position = transformation * vec4(position, 1.0f);
    //newColor = color;
    newNormals = normals;

    //Task 2d
    //gl_Position = vec4(position.x*-1, position.y*-1, position.z, 1.0f);
}