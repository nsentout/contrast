layout (location = 0) in vec4 color;
layout (location = 1) in float rotation;
layout (location = 2) in vec3 pos;

uniform mat4 projection;

out vec4 v_color;
out float v_rotation;

void main() {
  gl_Position = projection*vec4(pos, 1.);
  v_color = color;
  v_rotation= rotation;
}
