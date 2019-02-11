layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 size;
layout (location = 2) in vec4 color;
layout (location = 3) in float rotation;
layout (location = 4) in uint shape;
layout (location = 5) in float selection_angle;
layout (location = 6) in float start_radius;

uniform mat4 projection;

out vec2 v_size;
out vec4 v_color;
out uint v_shape;

void main() {
  gl_Position = projection * vec4(pos, 1.);
  v_size = size;
  v_color = color;
  v_shape = shape;
}
