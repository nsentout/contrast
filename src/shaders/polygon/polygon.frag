out vec4 out_color;
in vec4 f_color;
in vec2 g_uv;
in float size;
in float g_thick;
in float linelength;



void main() {
	/*float spacing = 1.5;
	float de = mod(g_uv.y + spacing, spacing*g_thick);
  float center = g_uv.y - de;
	if (center < fwidth(center))
	   discard;*/
	out_color = f_color;
}
