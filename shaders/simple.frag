#version 430 core
in vec3 newNormals;
in vec4 newColor;
out vec4 frag_color;




void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    float diff = max(dot(newNormals, -lightDirection), 0.0);
    vec4 normal_vec = newColor * diff;
    normal_vec[3] = 1.0;
    frag_color = normal_vec;
    //frag_color = vec4(normal_light, 1.0);
}
