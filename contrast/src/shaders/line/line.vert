layout (location = 0) in vec4 color;
layout (location = 1) in vec3 origin;
layout (location = 2) in vec3 target;
layout (location = 3) in vec3 previous;
layout (location = 4) in vec3 next;
layout (location = 5) in float thickness;

uniform mat4 projection;
out vec4 v_color;
out float v_rotation;
out float v_thick;
out uint v_mode;
out vec4 v_target;
out vec4 v_previous;
out vec4 v_next;

void main() {
  gl_Position = vec4(origin,1.0);
  v_target = vec4(target,1.0);
  v_previous = vec4(previous,1.0);
  v_next = vec4(next,1.0);
  v_color = color;
  v_thick = thickness;
}
