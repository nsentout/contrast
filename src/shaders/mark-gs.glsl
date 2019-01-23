layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in vec3 v_color[];
in vec2 v_size[];

out vec2 f_pos;
out vec2 f_size;
out vec3 f_color;

void build_quad(vec4 position)
{
    f_color = v_color[0];
    f_pos = position.xy;
    gl_Position = position + vec4(-0.2, -0.2, 0.0, 0.0);    // 1:bottom-left
    EmitVertex();
    gl_Position = position + vec4( 0.2, -0.2, 0.0, 0.0);    // 2:bottom-right
    EmitVertex();
    gl_Position = position + vec4(-0.2,  0.2, 0.0, 0.0);    // 3:top-left
    EmitVertex();
    gl_Position = position + vec4( 0.2,  0.2, 0.0, 0.0);    // 4:top-right
    EmitVertex();
    EndPrimitive();
}

void build_rect(vec4 position)
{
    f_color = v_color[0];
    f_size = v_size[0];
    f_pos = position.xy;

    gl_Position = position + vec4(-f_size.x, -f_size.y, 0.0, 0.0);    // 1:bottom-left
    EmitVertex();
    gl_Position = position + vec4(f_size.x, -f_size.y, 0.0, 0.0);    // 2:bottom-right
    EmitVertex();
    gl_Position = position + vec4(-f_size.x, f_size.y, 0.0, 0.0);    // 3:top-left
    EmitVertex();
    gl_Position = position + vec4(f_size.x, f_size.y, 0.0, 0.0);    // 4:top-right
    EmitVertex();
    EndPrimitive();
}

void main()
{
    //build_quad(gl_in[0].gl_Position);
    build_rect(gl_in[0].gl_Position);
}