#version 100
attribute highp vec2 position;
attribute highp vec2 tex_coord;

uniform vec2 screen_size;

varying vec2 v_tex_coord;

void main()
{
    gl_Position = vec4((position / screen_size * 2.0) - vec2(1.0, 1.0), 0.0, 1.0);
    v_tex_coord = tex_coord;
}