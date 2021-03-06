layout (location = 0) in vec4 color;
layout (location = 1) in float rotation;
layout (location = 2) in vec3 origin;
layout (location = 3) in vec3 target;
layout (location = 4) in vec3 previous;
layout (location = 5) in vec3 next;
layout (location = 6) in float thickness;
layout (location = 7) in vec3 polygon_centroid;
layout (location = 8) in int fill;

out vec4 v_color;
out float v_rotation;
out float v_thick;
out vec4 v_target;
out vec4 v_previous;
out vec4 v_next;
out vec3 v_centroid;
out int v_fill;

void main() {
  gl_Position = vec4(origin,1.0);
  v_target = vec4(target,1.0);
  v_previous = vec4(previous,1.0);
  v_next = vec4(next,1.0);
  v_color = color;
  v_rotation = rotation;
  v_thick = thickness;
  v_centroid = polygon_centroid;
  v_fill = fill;
}
