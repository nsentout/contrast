//#version 330 core


const float PI = 3.14159265358979323846264;
const float SQRT_2 = 1.4142135623730951;
const float TWO_PI = 2*PI;

flat in vec2 g_Size;     // Size of the node
flat in vec4 g_Color;    // Inner color
flat in uint g_Shape_front;
flat in uint g_Shape_back;
in vec2 g_uv;
flat in uvec4 g_id;
flat in float g_t;
flat in vec3 g_clipping;

out vec4 FragColor;
out uvec4 FragId;


// Begin code extracted from Rougier's page
// Union (A or B)
float csg_union(float d1, float d2)
{ return min(d1,d2); }

// Difference (A not B)
float csg_difference(float d1, float d2)
{ return max(d1,-d2); }

// Intersection (A and B)
float csg_intersection(float d1, float d2)
{  return max(d1,d2); }

// Exclusion (A xor B)
float csg_exclusion(float d1, float d2)
{ return min(max(d1,-d2), max(-d1,d2)); }
// End code extracted from Rougier's page





float regular_polygon(uint N) {

		// Angle and radius from the current pixel
		vec2 st = g_uv ;
		float a = atan(st.x,st.y)+PI;
		float r = TWO_PI/float(N);
		float d = cos(floor(.5+a/r)*r-a)*length(st)-0.5;
		return d;
		// XXX it seems derivation breaks everything
		float g = length( vec2(dFdx(d), dFdy(d)) );
		return d/g;
}

/**
 * Compute the distance to the shape.
 * g_uv contains the location in the shape
 */
float distance_shape(uint shape) {

	if(shape == 0u) { //circle (ellipse when elongated)
		/*
		float e1 = g_uv.x ;
		float e2 = g_uv.y ;
		float d = (e1*e1) + (e2*e2) - 1.0;
		float g = length( vec2(dFdx(d), dFdy(d)) );
		return d/g;
		*/
		return length(g_uv.xy) - 1.0;
	}

	else if (shape ==1u) {
		// draw a triangle
		return regular_polygon(3u);
	}

	else if (shape == 2u) {
		// unimplmented
	}

	else if (shape == 3u) { // rectangle
		vec2 p = g_uv.xy;
		float d = max(abs(p.x)-1., abs(p.y)-1.);
		float g = length( vec2(dFdx(d), dFdy(d)) );
		return d/g;
	}

	else if (shape == 4u) { // squircle
		float d2 = pow(abs(g_uv.x),4.0) + pow(abs(g_uv.y),4.0) - 1.0;
		float g2 = length( vec2(dFdx(d2),dFdy(d2)) );
		float f2 = d2/g2;
		return f2;
	}


	else if (shape == 5u) { //diamond
		float d = abs(g_uv.x)+ abs(g_uv.y)-1.0;
		float g = length(vec2(dFdx(d),dFdy(d)));
		return d/g;
	}

	else if (shape == 6u) { // donut
		float r1 = g_uv.x*g_uv.x+g_uv.y*g_uv.y - 1.0;
		float r2 = r1 + 0.7;
		float d = max(r1,-r2);
		float g = length(vec2(dFdx(d),dFdy(d)));
		return d/g;
	}



	else if (shape == 7u) { // pin
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




	else if (shape == 8u) 	{ // club
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

	else if (shape == 9u) { // heart
		float size=1.0;
		float x = SQRT_2/2.0 * (g_uv.x - g_uv.y);
		float y = SQRT_2/2.0 * (g_uv.x + g_uv.y);
		float r1 = max(abs(x),abs(y))-size/3.5;
		float r2 = length(g_uv - SQRT_2/2.0*vec2(+1.0,-1.0)*size/3.5)
			- size/3.5;
		float r3 = length(g_uv - SQRT_2/2.0*vec2(-1.0,-1.0)*size/3.5)
			- size/3.5;
		return min(min(r1,r2),r3);
	}

	else if (shape == 10u) { // Spade
		float size=1.0;
		// Reversed heart (diamond + 2 circles)
		float s= size * 0.85 / 3.5;
		float x = SQRT_2/2.0 * (g_uv.x + g_uv.y) + 0.4*s;
		float y = SQRT_2/2.0 * (g_uv.x - g_uv.y) - 0.4*s;
		float r1 = max(abs(x),abs(y)) - s;
		float r2 = length(g_uv - SQRT_2/2.0*vec2(+1.0,+0.2)*s) - s;
		float r3 = length(g_uv - SQRT_2/2.0*vec2(-1.0,+0.2)*s) - s;
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

	else if (shape==11u) { // Chevron
		float size=1.;
		float x = 1.0/SQRT_2 * (g_uv.x - g_uv.y);
		float y = 1.0/SQRT_2 * (g_uv.x + g_uv.y);
		float r1 = max(abs(x), abs(y)) - size/3.0;
		float r2 = max(abs(x-size/3.0), abs(y-size/3.0)) - size/3.0;
		return max(r1,-r2);

	}

	else if (shape == 12u) 	{ // clover
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


	else if (shape == 13u) 	{ // ring
		float size=1.0;
		float r1 = length(g_uv) - size/2.0;
		float r2 = length(g_uv) - size/4.0;
		return max(r1,-r2);
	}



	else if (shape == 14u) 	{ // tag
		float size=1;
		float r1 = max(abs(g_uv.x)- size/2.0, abs(g_uv.y)- size/6.0);
		float r2 = abs(g_uv.x-size/1.5)+abs(g_uv.y)-size;
		return max(r1,0.75*r2);
	}


	else if (shape == 15u) 	{ // cross
		float size = 1.0;
		float x = SQRT_2/2.0 * (g_uv.x - g_uv.y);
		float y = SQRT_2/2.0 * (g_uv.x + g_uv.y);
		float r1 = max(abs(x - size/3.0), abs(x + size/3.0));
		float r2 = max(abs(y - size/3.0), abs(y + size/3.0));
		float r3 = max(abs(x), abs(y));
		return max(min(r1,r2),r3) - size/2.0;
	}


	else if (shape == 16u) 	{ // Asterisk
		float size = 1.0;
		float x = SQRT_2/2.0 * (g_uv.x - g_uv.y);
		float y = SQRT_2/2.0 * (g_uv.x + g_uv.y);
		float r1 = max(abs(x)- size/2.0, abs(y)- size/10.0);
		float r2 = max(abs(y)- size/2.0, abs(x)- size/10.0);
		float r3 = max(abs(g_uv.x)- size/2.0, abs(g_uv.y)- size/10.0);
		float r4 = max(abs(g_uv.y)- size/2.0, abs(g_uv.x)- size/10.0);
		return min( min(r1,r2), min(r3,r4));
	}



	else if (shape == 17u) 	{ // Infinity
		float size = 1.0;
		const vec2 c1 = vec2(+0.2125, 0.00);
		const vec2 c2 = vec2(-0.2125, 0.00);
		float r1 = length(g_uv-c1*size) - size/3.5;
		float r2 = length(g_uv-c1*size) - size/7.5;
		float r3 = length(g_uv-c2*size) - size/3.5;
		float r4 = length(g_uv-c2*size) - size/7.5;
		return min( max(r1,-r2), max(r3,-r4));
	}


	else if (shape == 18u) 	{ // Arrow
		float size = 1.0;
		float x = g_uv.x;
		float y = g_uv.y;
		float r1 = abs(x) + abs(y) - size/2;
		float r2 = max(abs(x+size/2), abs(y)) - size/2;
		float r3 = max(abs(x-size/6)-size/4, abs(y)- size/4);
		return min(r3,max(.75*r1,r2));
	}

	//fallback to NONE shape => everything is outside shape
	return length(g_uv.xy);
}


/**
 * Filled representation of the shape
 */
vec4 filled(float distance, // Signed distance to line
		float linewidth, // Stroke line width
		float antialias, // Stroke antialiased area
		vec4 fill)
	// Fill color
{
	float t = linewidth / 2.0 - antialias;
	float signed_distance = distance;
	float border_distance = abs(signed_distance) - t;
	float alpha = border_distance / antialias;
	alpha = exp(-alpha * alpha);
	if( border_distance < 0.0 )
		return fill;
	else if( signed_distance < 0.0 )
		return fill;
	else
		return vec4(fill.rgb, alpha * fill.a);
}


vec4 stroke(float distance, // Signed distance to line
		float linewidth, // Stroke line width
		float antialias, // Stroke antialiased area
		vec4 stroke)
	// Stroke color
{
	float t = linewidth / 2.0 - antialias;
	float signed_distance = distance;
	float border_distance = abs(signed_distance) - t;
	float alpha = border_distance / antialias;
	alpha = exp(-alpha * alpha);
	if( border_distance < 0.0 )
		return stroke;
	else
		return vec4(stroke.rgb, stroke.a * alpha);
}

vec4 outline(float distance, // Signed distance to line
		float linewidth, // Stroke line width
		float antialias, // Stroke antialiased area
		vec4 stroke,
		// Stroke color
		vec4 fill)
// Fill color
{
	float t = linewidth / 2.0 - antialias;
	float signed_distance = distance;
	float border_distance = abs(signed_distance) - t;
	float alpha = border_distance / antialias;
	alpha = exp(-alpha * alpha);
	if( border_distance < 0.0 )
		return stroke;
	else if( signed_distance < 0.0 )
		return mix(fill, stroke, sqrt(alpha));
	else
		return vec4(stroke.rgb, stroke.a * alpha);
}
void main() {
    bool is_flat = true;
    float antialias = 1.0;

    // Manage radial clipping
    float angle = atan(g_uv.y, g_uv.x);
    float angle2 = mod(angle+TWO_PI, TWO_PI);
    float dAngle = min(fwidth(angle), fwidth(angle2));
    float radialStartAlpha = smoothstep(g_clipping.x-dAngle, g_clipping.x, angle);
    float radialStopAlpha = smoothstep(-g_clipping.y-dAngle, -g_clipping.y, -angle);
    float radialAlpha = 1.0;
    if(g_clipping.x > g_clipping.y) {
            radialAlpha = max(radialStartAlpha, radialStopAlpha);
    }
    else if (g_clipping.x < g_clipping.y) {
       radialAlpha = min(radialStartAlpha, radialStopAlpha);
    }

    // Manage inner clipping
    // TODO kind of rescaling is necessary i norer to work with percentage of shape
    float centerDist = length(g_uv.xy);
    float derivCDist = fwidth(centerDist);
    float internalClippingAlpha = smoothstep(-(g_clipping.z-derivCDist), -g_clipping.z, -centerDist);

    // real alpha
    float clippingAlpha = min(radialAlpha, internalClippingAlpha);
    // XXX For an unknown reason, alpha stuff do not work now :(
    if (clippingAlpha <= 0.0001) {
    	discard;
    }



    // Compute the shape
    float d_front = distance_shape(g_Shape_front);
    float d_back = distance_shape(g_Shape_back);
    float d = mix(d_front, d_back, g_t);

    float width = fwidth(d);

    if (d>width)  {
     	discard;
    }


    //FragColor =  filled(d, 0.1, .1, g_Color);
    vec4 expected_color = stroke(d, 0.01, antialias, g_Color);
    vec4 mark_color = mix(vec4(expected_color.xyz, 0.0), expected_color, clippingAlpha);
    FragColor =  mark_color;
    //FragColor =  outline(d, 0.01, .002, vec4(1.0, 1.0, 1.0, 0.0), g_Color);
    FragId = g_id;

    // specify the depth of the point
    // XXX is it usefull ?
    gl_FragDepth = gl_FragCoord.z / gl_FragCoord.w;
 // mark_id = vec4(0.0,0.0,1.0, 0.0);
}
