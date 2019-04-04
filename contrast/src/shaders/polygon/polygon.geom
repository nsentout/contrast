layout (points) in;
layout (triangle_strip, max_vertices = 4) out;


in vec2 v_size[];
in vec4 v_color[];
in float v_rotation[];
in float v_thick[];
in vec4 v_target[];
in vec4 v_previous[];
in vec4 v_next[];
in vec3 v_centroid[];
in int v_fill[];

out vec4 f_color;

uniform mat4 projection;

void build_line(vec4 position)
{
  f_color = v_color[0];
  float w = v_thick[0];

  vec2 t0 = normalize(position.xy - v_previous[0].xy);
  vec2 t1 = normalize(v_next[0].xy - v_target[0].xy);
  vec2 t2 = normalize(v_target[0].xy-position.xy);

  vec2 n0 = vec2(-t0.y, t0.x);
  vec2 n1 = vec2(-t2.y, t2.x);
  vec2 n2 = vec2(-t1.y, t1.x);
  vec2 miter;
  vec2 miter2;


  float dy;
  float dy2;
  if(v_fill[0] == 1){
    dy = distance(position.xy , v_centroid[0].xy);
    dy2 = distance(v_target[0].xy , v_centroid[0].xy);
    miter = normalize(position.xy-v_centroid[0].xy);
    miter2 = normalize(v_target[0].xy-v_centroid[0].xy);
  }
  else {
    miter = normalize(n0 + n1);
    miter2 = normalize(n1 + n2);
    float dot1 = dot(miter, n0);
    float dot2 = dot(miter2, n2);
    if(dot1 == 0.0){
      dot1 = 1.0;
    }
    if(dot2 == 0.0){
      dot2 = 1.0;
    }
    dy = w/dot1;
    dy2 = w/dot2;
  }

  float d1 = distance(position.xy, v_centroid[0].xy);
  float d2 = distance(v_target[0].xy, v_centroid[0].xy);
  if(dy < 0.0){
    dy = -dy;
  }
  if(dy2 < 0.0){
    dy2 = -dy2;
  }

  if( d1 <= dy){
    dy = d1;
  }
  if( d2 <= dy2) {
    dy2 = d2;
  }

  if (distance(position, vec4(v_centroid[0],0)) > distance(position + vec4(miter,0,0)*0.001, vec4(v_centroid[0],0))) {
    gl_Position = projection*(position + vec4(miter,0,0)*dy);
    EmitVertex();

    gl_Position = projection*(v_target[0] + vec4(miter2,0,0)*dy2);
    EmitVertex();

    gl_Position = projection*(position);
    EmitVertex();

    gl_Position = projection*(v_target[0]);
    EmitVertex();
  }
  else {
    gl_Position = projection*(position);
    EmitVertex();

    gl_Position = projection*(v_target[0]);
    EmitVertex();

    gl_Position = projection*(position - vec4(miter,0,0)*dy);
    EmitVertex();

    gl_Position = projection*(v_target[0] - vec4(miter2,0,0)*dy2);
    EmitVertex();
  }

  EndPrimitive();
}

void main()
{
  build_line(gl_in[0].gl_Position);
}
