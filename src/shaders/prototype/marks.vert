//#version 330 core

layout (location = 0) in vec3 a_pos_buff0;
layout (location = 1) in vec2 a_size_buff0;
layout (location = 2) in vec4 a_color_buff0;
layout (location = 3) in uint a_shape_buff0;
layout (location = 4) in float a_rotation_buff0;

layout (location = 5) in vec3 a_pos_buff1;
layout (location = 6) in vec2 a_size_buff1;
layout (location = 7) in vec4 a_color_buff1;
layout (location = 8) in uint a_shape_buff1;
layout (location = 9) in float a_rotation_buff1;

layout (location = 10) in vec3 a_pos_buff2;
layout (location = 11) in vec2 a_size_buff2;
layout (location = 12) in vec4 a_color_buff2;
layout (location = 13) in uint a_shape_buff2;
layout (location = 14) in float a_rotation_buff2;


uniform uint buffer_order;
uniform float t;

//TODO add real mark id ??

out vec2 v_size;
out vec4 v_color;
out float v_rotation;
flat out uint v_shape_front;
flat out uint v_shape_back;
flat out uvec4 v_id;
flat out float v_t;
flat out vec3 v_clipping;


void main() {


	vec3 pos_front;
	vec2 size_front;
	vec4 color_front;
	uint shape_front;
	float rotation_front;

	vec3 pos_back;
	vec2 size_back;
	vec4 color_back;
	uint shape_back;
	float rotation_back;

	// Set the variables depending on the real value
	if (0u == buffer_order ) {
		pos_front = a_pos_buff0;
		size_front = a_size_buff0;
		color_front = a_color_buff0;
		shape_front = a_shape_buff0;
		rotation_front = a_rotation_buff0;

		pos_back = a_pos_buff1;
		size_back = a_size_buff1;
		color_back = a_color_buff1;
		shape_back = a_shape_buff1;
		rotation_back = a_rotation_buff1;
	}

	else if (1u == buffer_order) {
		pos_front = a_pos_buff2;
		size_front = a_size_buff2;
		color_front = a_color_buff2;
		shape_front = a_shape_buff2;
		rotation_front = a_rotation_buff2;

		pos_back = a_pos_buff0;
		size_back = a_size_buff0;
		color_back = a_color_buff0;
		shape_back = a_shape_buff0;
		rotation_back = a_rotation_buff0;
	}
	else /*if (2u == order)*/ {
		pos_front = a_pos_buff1;
		size_front = a_size_buff1;
		color_front = a_color_buff1;
		shape_front = a_shape_buff1;
		rotation_front = a_rotation_buff1;

		pos_back = a_pos_buff2;
		size_back = a_size_buff2;
		color_back = a_color_buff2;
		shape_back = a_shape_buff2;
		rotation_back = a_rotation_buff2;
	}


  gl_Position = vec4(mix(pos_front, pos_back, t), 1);
  v_size = mix(size_front, size_back, t);
  v_color = mix(color_front, color_back, t);
  v_shape_front = shape_front;
  v_shape_back = shape_back;
  v_rotation = mix(rotation_front, rotation_back, t);

  // start angle, stop angle, innner
  vec3 clipping = vec3(0., 0., 0.22); // TODO use node's clipping information
  v_clipping = clipping;
  v_t = t;

  int id = gl_VertexID + 1; // TODO Check that it is enough to do that
  v_id = uvec4(
      (id /256 / 256 / 256) % 256,
      (id /256 / 256) % 256,
      (id /256) % 256,
      id % 256
   );
}
