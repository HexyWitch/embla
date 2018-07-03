#version 100
varying highp vec2 v_tex_coord;

uniform highp vec2 texture_size;
uniform sampler2D texture;

void main()
{
    gl_FragColor = texture2D(texture, v_tex_coord / texture_size);
}