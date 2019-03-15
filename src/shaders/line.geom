layout (points) in;
layout (triangle_strip, max_vertices = 4) out;




in vec2 v_size[];
in vec4 v_color[];
in float v_rotation[];
in float v_thick[];
in uint v_mode[];
in vec4 v_target[];
in vec4 v_previous[];


out vec4 f_color;
out vec4 g_uv;

uniform mat4 projection;

void build_line(vec4 position)
{
  f_color = v_color[0];

  vec2 T = normalize(v_target[0].xy-position.xy);
  vec2 O = vec2(-T.y , T.x);

  gl_Position = projection*(position + vec4(O,0,0)*v_thick[0]*0.5);
  g_uv = gl_Position;
  EmitVertex();
  gl_Position = projection*(position - vec4(O,0,0)*v_thick[0]*0.5);
  g_uv = gl_Position;
  EmitVertex();
  gl_Position = projection*(v_target[0] + vec4(O,0,0)*v_thick[0]*0.5);
  g_uv = gl_Position;
  EmitVertex();
  gl_Position = projection*(v_target[0] - vec4(O,0,0)*v_thick[0]*0.5);
  g_uv = gl_Position;
  EmitVertex();

  EndPrimitive();
}

void main()
{
  build_line(gl_in[0].gl_Position);
}
