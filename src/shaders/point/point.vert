layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 target_pos;
layout (location = 2) in float start_pos;
layout (location = 3) in vec2 size;
layout (location = 4) in vec2 target_size;
layout (location = 5) in float start_size;
layout (location = 6) in vec4 color;
layout (location = 7) in vec4 target_color;
layout (location = 8) in float start_color;
layout (location = 9) in float rotation;
layout (location = 10) in float target_rotation;
layout (location = 11) in float start_rotation;
layout (location = 12) in uint shape;
layout (location = 13) in uint target_shape;
layout (location = 14) in float start_shape;

uniform mat4 projection;

out vec2 v_size;
out vec4 v_color;
out float v_rotation;
flat out uint v_shape;
flat out uint v_target_shape;
out float v_start_shape;

uniform float t;


void main() {
    gl_Position = mix(vec4(pos, 1.), vec4(target_pos, 1.), min(t - start_pos, 1.0));
    v_size = mix(size, target_size, min(t - start_size, 1.0));
    v_color = mix(color, target_color, min(t - start_color, 1.0));
    v_rotation = mix(rotation, target_rotation, min(t - start_rotation, 1.0));
    v_shape = shape;
    v_target_shape = target_shape;
    v_start_shape = start_shape;
}
