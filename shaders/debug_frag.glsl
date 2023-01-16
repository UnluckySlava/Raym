#version 330 core

out vec4 FragColor;

uniform vec2 resolution;

void main() {
    vec2 uv = (gl_FragCoord.xy * 2.0 - resolution) / resolution.y;
    FragColor = vec4(uv, 0.0, 1.0);
}