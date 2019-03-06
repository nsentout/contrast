
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

     else if (shape == 1u) {//Rectangle

        return 0;
     }

     else if (shape == 2u) {//Triangle
        float x= M_SQRT_2/2.0 * (g_uv.x-g_uv.y);
        float y= M_SQRT_2/2.0 * (g_uv.x+g_uv.y);

        float r1=max(abs(x), abs(y)) - 1/(2.0*M_SQRT_2);
        float r2=g_uv.y;
        return max(r1, r2);

          }

     else if (shape == 3u) { //Disc and Elipse

             return length(g_uv.xy)-0.6;

               }
     else if (shape == 4u) { //Point ?

                  return length(g_uv.xy);

                    }

     else if (shape == 5u) { //Squircle

         float d2 = pow(abs(g_uv.x),4.0) + pow(abs(g_uv.y),4.0) - 1.0;
               		float g2 = length( vec2(dFdx(d2),dFdy(d2)) );
               		float f2 = d2/g2;
               		return f2;
     }

     else if (shape == 6u) { //Diamond
     		float d = abs(g_uv.x)+ abs(g_uv.y)-1.0;
     		float g = length(vec2(dFdx(d),dFdy(d)));
     		return d/g;
     	}

     else if (shape == 7u) { //Donut
     		float r1 = g_uv.x*g_uv.x+g_uv.y*g_uv.y - 0.9;
     		float r2 = r1 + 0.7;
     		float d = max(r1,-r2);
     		float g = length(vec2(dFdx(d),dFdy(d)));
     		return d/g;
     	}

     else if (shape == 8u) { //Pin
     		float size=1.;
     		vec2 c1 = vec2(0.0,-0.15)*size;
     		float r1 = length(g_uv-c1)-size/2.675;
     		vec2 c2 = vec2(+1.49,-0.80)*size;
     		float r2 = length(g_uv-c2) - 2.*size;
     		vec2 c3 = vec2(-1.49,-0.80)*size;
     		float r3 = length(g_uv-c3) - 2.*size;
     		float r4 = length(g_uv-c1)-size/5;
     		return max( min(r1,max(max(r2,r3),-g_uv.y)), -r4);

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
