layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 size;
layout (location = 2) in vec3 color;
layout (location = 3) in uint shape;

out vec3 v_color;
out vec2 v_size;  // width and height of the rectangle
out uint v_shape;

void main()
{
   gl_Position = vec4(pos, 0., 1.);
   v_color = color;
   v_size = size;
   v_shape = shape;
}