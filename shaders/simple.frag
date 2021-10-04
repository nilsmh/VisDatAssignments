#version 430 core
in vec3 newNormals;
//in vec4 newColor;
out vec4 frag_color;
// out vec3 FragNormals;



void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    float diff = max(dot(newNormals, -lightDirection), 0.0);
    vec3 normal_light = newNormals * diff;
    frag_color = vec4(normal_light, 1.0);
}
