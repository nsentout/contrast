layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in vec2 v_size[];
in vec4 v_color[];
in uint v_shape[];

out vec3 f_pos;
out vec2 f_size;
out vec4 f_color;

void build_simple_shape(vec4 position)
{
    f_color = v_color[0];
    f_size = v_size[0];
    f_pos = position.xyz;

    if (v_shape[0] == 0u) { // TODO: utiliser la fonction distance
        // NONE
    }
    else if (v_shape[0] == 1u) {    // RECTANGLE
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
    else if (v_shape[0] == 2u) {    // TRIANGLE
        gl_Position = position + vec4(0, f_size.y, 0.0, 0.0);
        EmitVertex();
        gl_Position = position + vec4(-f_size.x, -f_size.y, 0.0, 0.0);
        EmitVertex();
        gl_Position = position + vec4(f_size.x, -f_size.y, 0.0, 0.0);
        EmitVertex();
        EndPrimitive();
    }
}

void main()
{
    build_simple_shape(gl_in[0].gl_Position);
}