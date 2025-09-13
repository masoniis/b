#version 330 core

// Input vertex data (from the VBO)
layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTexCoord;

// Output to fragment shader
out vec2 v_tex_coord;

uniform mat4 projection;

void main() {
  gl_Position = projection * vec4(aPos.x, aPos.y, 0.0, 1.0);
  v_tex_coord = aTexCoord;
}
