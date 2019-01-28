in vec2 f_pos;
in vec3 f_color;

out vec4 frag;

void main()
{
    frag = vec4(f_color, 1.);
}