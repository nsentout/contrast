//#version 330 core

/**
 * The aim of the geometry shader is to generate the bounding rectangle of each node.
 * Remember a node is ony represented by a vertex (its center or its upper corner -- to be defined)
 */

// We read points ...
 layout(points) in;

// ... adn output triangles
 layout(triangle_strip, max_vertices=6) out;

in vec4 v_pos[];
in vec2 v_size[];
in vec4 v_color[];
in float v_rotation[];
flat in uint v_shape_front[];
flat in uint v_shape_back[];
flat in uvec4 v_id[];
flat in vec3 v_clipping[];
flat in float v_t[];

flat out vec2 g_Size;	// Size of the node
flat out vec4 g_Color;	// Inner color
flat out uint g_Shape_front;	// Shape
flat out uint g_Shape_back;	// Shape
flat out uvec4 g_id;
flat out vec3 g_clipping;
flat out float g_t;

out vec2 g_uv; // position in the mark texture

uniform mat4 mvp;




// TODO récupérer les matrices de transformations et les transférer

/**
 * Transform a point as a pair of triangles.
 *
 *      width
 *   <---------->
 *   +----------+ ^
 *   |\         | | h
 *   |  \    t1 | | e
 *   |    c     | | i
 *   |      \   | | g
 *   | t2     \ | | h
 *   +----------+ v t
 */

void main() {
	// discard invisible marks
	if (v_color[0].a != 1.0 && v_size[0].x >0.0 && v_size[0].y > 0.0) {

		vec4 nodeCenter = gl_in[0].gl_Position;

		float mid_width =  v_size[0].x / 2.0f;
		float mid_height = v_size[0].y / 2.0f;

		float cos_rotation = cos(v_rotation[0]);
		float sin_rotation = sin(v_rotation[0]);


		// Common for all the generated points
		g_Size = v_size[0];
		g_Color = v_color[0];
		g_Shape_front = v_shape_front[0];
		g_Shape_back = v_shape_back[0];
		g_id = v_id[0];
		g_clipping = v_clipping[0];
		g_t = v_t[0];

		// Build the transformation table
		// Sadly triangle_fan like struct cannot be used :(
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
			g_uv = deltas[i]  / (g_Size.xy/2.0);

			// point in screen space (rotated)
			float x = (deltas[i].x)*cos_rotation - (deltas[i].y)*sin_rotation;
			float y = sin_rotation*(deltas[i].x) + cos_rotation*(deltas[i].y);
			vec4 rotated_delta = vec4(x ,y, 0, 0);
			gl_Position =  mvp * (nodeCenter + rotated_delta);
			EmitVertex();
		}

		EndPrimitive();
}
}
