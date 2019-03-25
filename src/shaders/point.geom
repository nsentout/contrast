layout (points) in;
layout (triangle_strip, max_vertices = 6) out;

in vec2 v_size[];
in vec4 v_color[];
flat in uint v_shape[];
flat in uint v_target_shape[];
in float v_start_shape[];
in float v_rotation[];

out vec4 f_color;
flat out uint f_shape;
flat out uint f_target_shape;
out float f_start_shape;
out vec2 g_uv;

uniform mat4 projection;

void build_simple_shape(vec4 position)
{
    vec4 nodeCenter = gl_in[0].gl_Position;

    float mid_width=v_size[0].x/2.0f;
    float mid_height=v_size[0].y/2.0f;

    float cos_rotation = cos(v_rotation[0]);
    float sin_rotation = sin(v_rotation[0]);

    f_color = v_color[0];
    f_shape = v_shape[0];
    f_target_shape = v_target_shape[0];
    f_start_shape = v_start_shape[0];


	vec2 deltas[6] = vec2[6](
        vec2( mid_width, -mid_height),
        vec2( mid_width,  mid_height),
        vec2(-mid_width,  mid_height),
        vec2( mid_width, -mid_height),
        vec2(-mid_width, -mid_height),
        vec2(-mid_width,  mid_height)
	);

	for(int i=0 ; i<6; ++i){
		// point in object space (not rotated)
		g_uv = deltas[i]  / (v_size[0].xy/2.0);

		// point in screen space (rotated)
		float x = (deltas[i].x)*cos_rotation - (deltas[i].y)*sin_rotation;
		float y = sin_rotation*(deltas[i].x) + cos_rotation*(deltas[i].y);
		vec4 rotated_delta = vec4(x ,y, 0, 0);
		gl_Position = projection* (nodeCenter + rotated_delta);
		EmitVertex();
	}

	EndPrimitive();
}

void main()
{
	build_simple_shape(gl_in[0].gl_Position);
}
