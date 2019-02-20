
const float M_SQRT_2 = 1.4142135623730951;

in vec3 f_pos;
in vec2 f_size;
in vec4 f_color;
flat in uint f_shape;
in vec2 g_uv;
out vec4 frag;

float distance_shape(uint shape)
{
    if(shape == 0u)
    {
        return 100;
    }

     else if (shape == 1u) {

        return 0;
     }

     else if (shape == 2u) {
        float x= M_SQRT_2/2.0 * (g_uv.x-g_uv.y);
        float y= M_SQRT_2/2.0 * (g_uv.x+g_uv.y);

        float r1=max(abs(x), abs(y)) - 1/(2.0*M_SQRT_2);
        float r2=g_uv.y;
        return max(r1, r2);

          }

     else if (shape == 3u) {

             return length(g_uv.xy)-0.6;

               }
     return 0;
}

void main()
{
    float d =  distance_shape(f_shape);
    float width=fwidth(d);
    if(d>width)
    {
        discard;
    }
  frag = f_color;
}
