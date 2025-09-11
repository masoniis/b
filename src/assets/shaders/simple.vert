#version 330 core

// Input vertex data (from the VBO)
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;

// Output to fragment shader
out vec2 v_tex_coord;

uniform mat4 projection;
uniform mat4 modelView;

void main() {
  gl_Position = projection * modelView * vec4(aPos, 1.0);
  v_tex_coord = aTexCoord;
}