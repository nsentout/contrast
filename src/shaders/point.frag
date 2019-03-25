const float PI = 3.14159265358979323846264;
const float M_SQRT_2 = 1.4142135623730951;
const float TWO_PI = 2*PI;

in vec4 f_color;
flat in uint f_shape;
flat in uint f_target_shape;
in float f_start_shape;

in vec2 g_uv;
out vec4 frag;

uniform float t;

float distance_shape(uint shape)
{
    if(shape == 0u){
        return length(g_uv.xy);
    }

    else if (shape == 1u) {//Rectangle
        return 0.;
    }

    else if (shape == 2u) {//Triangle
        vec2 st = g_uv ;
        float a = atan(st.x,st.y)+PI;
        float r = TWO_PI/float(3u);
        float d = cos(floor(.5+a/r)*r-a)*length(st)-0.5;
        return d;
    }

    else if (shape == 3u) { //Disc and Elipse
        return length(g_uv.xy)-0.6;
    }

    else if (shape == 4u) { //Point
        return length(g_uv.xy)-0.1;
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

    else if (shape == 9u) { //Club
        float size = 1.0;
        // clover (3 discs)
        const float t1 = -PI/2.0;
        const vec2 c1 = 0.225*vec2(cos(t1),sin(t1));
        const float t2 = t1+2*PI/3.0;
        const vec2 c2 = 0.225*vec2(cos(t2),sin(t2));
        const float t3 = t2+2*PI/3.0;
        const vec2 c3 = 0.225*vec2(cos(t3),sin(t3));
        float r1 = length( g_uv - c1*size) - size/4.25;
        float r2 = length( g_uv - c2*size) - size/4.25;
        float r3 = length( g_uv - c3*size) - size/4.25;
        float r4 = min(min(r1,r2),r3);

        // Root (2circles and 2 half-planes)
        const vec2 c4 = vec2(+0.65, 0.125);
        const vec2 c5 = vec2(-0.65, 0.125);
        float r5 =length(g_uv-c4*size) - size/1.6;
        float r6 =length(g_uv-c5*size) - size/1.6;
        float r7 =g_uv.y - 0.5*size;
        float r8 =0.2*size - g_uv.y;
        float r9 =max(-min(r5,r6), max(r7,r8));
        return min(r4,r9);
    }

    else if (shape == 10u) { //Heart
        float size=1.0;
        float x = M_SQRT_2/2.0 * (g_uv.x - g_uv.y);
        float y = M_SQRT_2/2.0 * (g_uv.x + g_uv.y);
        float r1 = max(abs(x),abs(y))-size/3.5;
        float r2 = length(g_uv - M_SQRT_2/2.0*vec2(+1.0,-1.0)*size/3.5)- size/3.5;
        float r3 = length(g_uv - M_SQRT_2/2.0*vec2(-1.0,-1.0)*size/3.5)- size/3.5;
        return min(min(r1,r2),r3);
    }

    else if (shape == 11u) { //Spade
        float size=1.0;
        // Reversed heart (diamond + 2 circles)
        float s= size * 0.85 / 3.5;
        float x = M_SQRT_2/2.0 * (g_uv.x + g_uv.y) + 0.4*s;
        float y = M_SQRT_2/2.0 * (g_uv.x - g_uv.y) - 0.4*s;
        float r1 = max(abs(x),abs(y)) - s;
        float r2 = length(g_uv - M_SQRT_2/2.0*vec2(+1.0,+0.2)*s) - s;
        float r3 = length(g_uv - M_SQRT_2/2.0*vec2(-1.0,+0.2)*s) - s;
        float r4 = min(min(r1,r2),r3);

        // Root (2circles and 2 half-planes)
        const vec2 c1 = vec2(+0.65, 0.125);
        const vec2 c2 = vec2(-0.65, 0.125);
        float r5 =length(g_uv-c1*size) - size/1.6;
        float r6 =length(g_uv-c2*size) - size/1.6;
        float r7 =g_uv.y - 0.5*size;
        float r8 =0.1*size - g_uv.y;
        float r9 =max(-min(r5,r6), max(r7,r8));
        return min(r4,r9);
    }

    else if (shape==12u) { //Chevron
        float size=1.;
        float x = 1.0/M_SQRT_2 * (g_uv.x - g_uv.y);
        float y = 1.0/M_SQRT_2 * (g_uv.x + g_uv.y);
        float r1 = max(abs(x), abs(y)) - size/3.0;
        float r2 = max(abs(x-size/3.0), abs(y-size/3.0)) - size/3.0;
        return max(r1,-r2);

    }

    else if (shape == 13u) 	{ //Clover
        float size = 1.0;
        // clover (3 discs)
        const float t1 = -PI/2.0;
        const vec2 c1 = 0.225*vec2(cos(t1),sin(t1));
        const float t2 = t1+2*PI/3.0;
        const vec2 c2 = 0.225*vec2(cos(t2),sin(t2));
        const float t3 = t2+2*PI/3.0;
        const vec2 c3 = 0.225*vec2(cos(t3),sin(t3));
        float r1 = length( g_uv - c1*size) - size/4.25;
        float r2 = length( g_uv - c2*size) - size/4.25;
        float r3 = length( g_uv - c3*size) - size/4.25;
        return  min(min(r1,r2),r3);

    }

    else if (shape == 14u) 	{ // ring
        float size=1.0;
        float r1 = length(g_uv) - size/2.0;
        float r2 = length(g_uv) - size/4.0;
        return max(r1,-r2);
    }

    else if (shape == 15u) 	{ //Tag
        float size=1;
        float r1 = max(abs(g_uv.x)- size/2.0, abs(g_uv.y)- size/6.0);
        float r2 = abs(g_uv.x-size/1.5)+abs(g_uv.y)-size;
        return max(r1,0.75*r2);
    }

    else if (shape == 16u) 	{ //Cross
        float size = 1.0;
        float x = M_SQRT_2/2.0 * (g_uv.x - g_uv.y);
        float y = M_SQRT_2/2.0 * (g_uv.x + g_uv.y);
        float r1 = max(abs(x - size/3.0), abs(x + size/3.0));
        float r2 = max(abs(y - size/3.0), abs(y + size/3.0));
        float r3 = max(abs(x), abs(y));
        return max(min(r1,r2),r3) - size/2.0;
    }

    else if (shape == 17u) 	{ //Asterisk
        float size = 1.0;
        float x = M_SQRT_2/2.0 * (g_uv.x - g_uv.y);
        float y = M_SQRT_2/2.0 * (g_uv.x + g_uv.y);
        float r1 = max(abs(x)- size/2.0, abs(y)- size/10.0);
        float r2 = max(abs(y)- size/2.0, abs(x)- size/10.0);
        float r3 = max(abs(g_uv.x)- size/2.0, abs(g_uv.y)- size/10.0);
        float r4 = max(abs(g_uv.y)- size/2.0, abs(g_uv.x)- size/10.0);
        return min( min(r1,r2), min(r3,r4));
    }

    else if (shape == 18u) 	{ //Infinity
        float size = 1.0;
        const vec2 c1 = vec2(+0.2125, 0.00);
        const vec2 c2 = vec2(-0.2125, 0.00);
        float r1 = length(g_uv-c1*size) - size/3.5;
        float r2 = length(g_uv-c1*size) - size/7.5;
        float r3 = length(g_uv-c2*size) - size/3.5;
        float r4 = length(g_uv-c2*size) - size/7.5;
        return min( max(r1,-r2), max(r3,-r4));
    }

    else if (shape == 19u) 	{ //Arrow
        float size = 1.0;
        float x = g_uv.x;
        float y = g_uv.y;
        float r1 = abs(x) + abs(y) - size/2;
        float r2 = max(abs(x+size/2), abs(y)) - size/2;
        float r3 = max(abs(x-size/6)-size/4, abs(y)- size/4);
        return min(r3,max(.75*r1,r2));
    }

    return length(g_uv.xy);
}

float interpolation_distance_shape() {
    return mix(distance_shape(f_shape), distance_shape(f_target_shape), min(t - f_start_shape, 1.0));
}

void main()
{
    float d = interpolation_distance_shape();

    float width=fwidth(d);
    if(d>width)
    {
        discard;
    }
    frag = f_color;
}
