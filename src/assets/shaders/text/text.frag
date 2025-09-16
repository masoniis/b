#version 330 core

// Input from vertex shader
in vec2 v_tex_coord;

// Output to the framebuffer
out vec4 FragColor;

// The texture sampler for the font atlas
uniform sampler2D u_texture;
// The color of the text
uniform vec4 u_textColor;

void main()
{
    // The red channel of the texture contains the alpha value for the font
    float alpha = texture(u_texture, v_tex_coord).r;
    // Combine the text color with the alpha from the font atlas
    FragColor = vec4(u_textColor.rgb, u_textColor.a * alpha);
}
