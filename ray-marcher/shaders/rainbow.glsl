#version 330 core
#define PI 3.14159265359
out vec4 FragColor;
uniform float iTime;

void main()
{
    vec2 uv = gl_FragCoord.xy/vec2(800, 600) * 2.0 - 1.0;
    
    float offset_col = 2.0*PI/3.0;
    float rbForm = uv.x * 10.0;
    vec3 col = vec3(0.0);
    col.r = cos(rbForm + 4.0*PI/3.0 +offset_col);
    col.g = cos(rbForm + 2.0*PI/3.0 +offset_col);
    col.b = cos(rbForm              +offset_col);
    col += vec3(1.0);
    col *= 0.5;
    col = mix(col, vec3(1.0), pow(abs(uv.x) + clamp(sin(iTime * 2.0 + 1.5), 0.0, 1.0), 4.0));
    FragColor = vec4(col, 1.0);
}