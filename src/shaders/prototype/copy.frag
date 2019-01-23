#version 330 core

in vec2 v_uv;

out vec4 frag;

uniform sampler2D source_texture;

void main() {
  frag = texture(source_texture, v_uv).rgba;
}
