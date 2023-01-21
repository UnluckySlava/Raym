#version 330 core
out vec4 FragColor;

const int MAX_STEPS = 255;
const float MIN_DIST = 0.0001;
const float MAX_DIST = 50.0;
const float EPSILON = 0.001;

uniform vec3 cameraPos;
uniform vec3 cameraRot;
uniform vec2 resolution;

float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0));
	return max(r, min (a, b)) - length(u);
}

float vmax(vec2 v) {
	return max(v.x, v.y);
}

float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}

float vmax(vec4 v) {
	return max(max(v.x, v.y), max(v.z, v.w));
}

float fBoxCheap(vec3 p, vec3 b) { //cheap box
	return vmax(abs(p) - b);
}
float fBox(vec3 p, vec3 b) {
	vec3 d = abs(p) - b;
	return length(max(d, vec3(0))) + vmax(min(d, vec3(0)));
}

float fSphere(vec3 p, float r) {
	return length(p) - r;
}
float fPlane(vec3 p, vec3 n, float distanceFromOrigin) {
	return dot(p, n) + distanceFromOrigin;
}

float map(in vec3 pos) {
    float sphere = fSphere(pos, 1.0);
    float plane = fPlane(pos, vec3(0.0, 1.0, 0.0), 1.0);
    float result = min(plane, sphere);
    return result;
}

vec3 calcNormal(in vec3 pos) {
    vec2 e = vec2(0.0001,0.0);
    return normalize(vec3( 
        map(pos + e.xyy) - map(pos - e.xyy),
		map(pos + e.yxy) - map(pos - e.yxy),
		map(pos + e.yyx) - map(pos - e.yyx)));
}
float rayMarch(vec3 ro, vec3 rd) {
    float t = 0.0;
    for (int i=0; i < MAX_STEPS; i++) {
        vec3 pos = ro + t * rd;
        float h = map(pos);
        if (h < EPSILON)
            break;
        t += h;
        if (t > MAX_DIST){
            t = -1.0;
            break;
        }
    }
    return t;
}

void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

void main() {
    vec2 uv = (gl_FragCoord.xy * 2.0 - resolution.xy) / resolution.y;
    vec3 rd = normalize(vec3(uv, -1.5));
    pR(rd.xz, cameraRot.y);
    pR(rd.yx, cameraRot.x * sin(cameraRot.y));
    pR(rd.yz, cameraRot.x * cos(cameraRot.y));
	vec3 col = vec3(0.0, 0.0, 0.0);

    float t = rayMarch(cameraPos, rd);
    if (t > 0.0) {
        vec3 pos = cameraPos + t * rd;
        vec3 nor = calcNormal(pos);
        vec3 sun = normalize(vec3(0.8, 0.4, 0.2));
        float sun_dif = clamp(dot(nor, sun), 0.0, 1.0);
        float sun_sha = step(rayMarch(pos+nor * 0.001, sun), 0.0);
        float sky_dif = clamp(0.5 + 0.5 * dot(nor, vec3(0.0, 1.0, 0.0)), 0.0, 1.0);
        col  = vec3(1.0) * sun_dif * sun_sha;
        col += vec3(0.0, 0.1, 0.3) * sky_dif;
    }
    col = pow(col, vec3(0.4545));
    FragColor = vec4(col, 1.0);
}