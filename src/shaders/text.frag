in vec2 TexCoords;
out vec4 out_color;

uniform vec4 color;
uniform sampler2D atlas;

void main()
{
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(atlas, TexCoords).r);
    out_color = color * sampled;
}
