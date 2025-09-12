#version 330 core

// Input from vertex shader
in vec2 v_tex_coord;

// Output to the framebuffer
out vec4 FragColor;

// The texture sampler
uniform sampler2D u_texture;

void main()
{
		// TODO: texture properly
    // FragColor = texture(u_texture, v_tex_coord);
    FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Output solid red color
}
