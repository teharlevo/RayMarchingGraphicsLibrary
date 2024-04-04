#version 330 core

struct transform{
    vec3 pos;
    vec3 rot;
    float scale;
    int type;
};

uniform vec3 camareAngles;
uniform vec3 camarePos;
uniform int size;
uniform transform tran[200];

in vec2 fUV;

out vec4 Color;

mat3 rotateX(float theta) {
    float c = cos(theta);
    float s = sin(theta);
    return mat3(
        vec3(1, 0, 0),
        vec3(0, c, -s),
        vec3(0, s, c)
    );
}

// Rotation matrix around the Y axis.
mat3 rotateY(float theta) {
    float c = cos(theta);
    float s = sin(theta);
    return mat3(
        vec3(c, 0, s),
        vec3(0, 1, 0),
        vec3(-s, 0, c)
    );
}

// Rotation matrix around the Z axis.
mat3 rotateZ(float theta) {
    float c = cos(theta);
    float s = sin(theta);
    return mat3(
        vec3(c, -s, 0),
        vec3(s, c, 0),
        vec3(0, 0, 1)
    );
}

// Identity matrix.
mat3 identity() {
    return mat3(
        vec3(1, 0, 0),
        vec3(0, 1, 0),
        vec3(0, 0, 1)
    );
}

vec3 rotateVec3(vec3 p,vec3 angles){
    return identity() * rotateZ(angles.z) * rotateY(angles.y) * rotateX(angles.x) * p;
}

vec3 opTwist(vec3 p,float k)
{
    
    float c = cos(k*p.y);
    float s = sin(k*p.y);
    mat2  m = mat2(c,-s,s,c);
    vec3  q = vec3(m*p.xz,p.y);
    return q;
}

float sdSphere(vec3 p, float s )
{
  return length(p)-s;
}

float sdTorus( vec3 p, vec2 t )
{
  vec2 q = vec2(length(p.xz)-t.x,p.y);
  return length(q)-t.y;
}

float sdBox( vec3 p, vec3 b )
{
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdEllipsoid( vec3 p, vec3 r )
{
  float k0 = length(p/r);
  float k1 = length(p/(r*r));
  return k0*(k0-1.0)/k1;
}

//#dis funcans here#

float map(vec3 pos){
    float len = 10000;
    for (int i = 0; i < size; i++) {
        vec3 p = pos;
        p -= tran[i].pos;
                p = rotateVec3(p,tran[i].rot);
        p = p/tran[i].scale;
        float dis = len;
        if(tran[i].type == 0){
            dis = sdBox(p,vec3(1.0));
        }else if(tran[i].type == 1){
            dis = sdTorus(p,vec2(1.0,0.5));
        }
        else if(tran[i].type == 2){
            dis = sdSphere(p,1.0);
        }
        //#else if else if#
        len = min (dis * tran[i].scale,len);
    }

    return len;
}

vec3 palette(float t) {
    return .5+.5*cos(6.28318*(t+vec3(.3,.416,.557)));
}

mat2 rot2D(float a) {
    return mat2(cos(a), -sin(a), sin(a), cos(a));
}

vec3 getRayDir() {
    vec2 coords = fUV * 2.0 -1.0f;
    vec3 rayDir = normalize(vec3(rot2D(camareAngles.z) * coords,1));
    rayDir.xz = rayDir.xz * rot2D(camareAngles.x);
    rayDir = normalize(rayDir);
    rayDir.yz = rayDir.yz * rot2D(camareAngles.y);
    rayDir = normalize(rayDir);
 return rayDir;
}

void main()
{
    vec3 rayDir = getRayDir();
    float totalDis = 0.0;
    for (int i = 0; i < 80; i++) {
        vec3 p = camarePos + rayDir * totalDis;

        float dis = map(p);

        totalDis += dis;

        if (dis < .01 || totalDis > 500.) break;
        
    }

    if(totalDis < 500. ){
        Color = vec4(palette(10.0 - totalDis * 0.1), 1.0);
    }
    else{
        Color = vec4(vec3(0.0), 1.0);
    }
}
