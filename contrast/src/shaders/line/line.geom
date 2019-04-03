layout (points) in;
layout (triangle_strip, max_vertices = 4) out;


in vec4 v_color[];
in float v_thick[];
in vec4 v_target[];
in vec4 v_previous[];
in vec4 v_next[];


out vec4 f_color;
out vec2 g_uv;
out float size;

uniform mat4 projection;

void build_line(vec4 position)
{
  f_color = v_color[0];
  float w = v_thick[0]/2.0;

  vec2 t0 = normalize(position.xy - v_previous[0].xy);
  vec2 t1 = normalize(v_next[0].xy - v_target[0].xy);
  vec2 t2 = normalize(v_target[0].xy-position.xy);

  float l = length(v_target[0].xy-position.xy);
  vec2 O = vec2(-t2.y , t2.x);
  size = v_thick[0];


  vec2 n0 = vec2(-t0.y, t0.x);
  vec2 n1 = vec2(-t2.y, t2.x);
  vec2 n2 = vec2(-t1.y, t1.x);
  vec2 miter = normalize(n0 + n1);
  vec2 miter2 = normalize(n1 + n2);


  float dy = w/dot(miter, n0);
  float dy2 = w/dot(miter2, n2);

  if (v_previous[0].xy == position.xy) {
    gl_Position = projection*(position + vec4(O,0,0)*v_thick[0]*0.5);
    g_uv = vec2(-v_thick[0]/2,-l/2);
    EmitVertex();
    gl_Position = projection*(position - vec4(O,0,0)*v_thick[0]*0.5);
    g_uv = vec2(v_thick[0]/2,-l/2);
    EmitVertex();
  } else {
    gl_Position = projection*(position + vec4(miter,0,0)*dy);
    g_uv = vec2(-v_thick[0]/2,-l/2);
    EmitVertex();
    gl_Position = projection*(position - vec4(miter,0,0)*dy);
    g_uv = vec2(v_thick[0]/2,-l/2);
    EmitVertex();
  }
  if (v_target[0].xy == v_next[0].xy){
    gl_Position = projection*(v_target[0] + vec4(O,0,0)*v_thick[0]*0.5);
    g_uv = vec2(-v_thick[0]/2,l/2);
    EmitVertex();
    gl_Position = projection*(v_target[0] - vec4(O,0,0)*v_thick[0]*0.5);
    g_uv = vec2(v_thick[0]/2,l/2);
    EmitVertex();
  } else {
    gl_Position = projection*(v_target[0] + vec4(miter2,0,0)*dy2);
    g_uv = vec2(-v_thick[0]/2,l/2);
    EmitVertex();
    gl_Position = projection*(v_target[0] - vec4(miter2,0,0)*dy2);
    g_uv = vec2(v_thick[0]/2,l/2);
    EmitVertex();
  }

  EndPrimitive();
}

void main()
{
  build_line(gl_in[0].gl_Position);
}
