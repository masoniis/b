#version 330 core

// Input from vertex shader
in vec2 v_tex_coord;

// Output to the framebuffer
out vec4 FragColor;

// The texture sampler
uniform sampler2D u_texture;

void main()
{
    FragColor = texture(u_texture, v_tex_coord);
}