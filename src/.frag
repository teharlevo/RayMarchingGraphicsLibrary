#version 330 core

struct transform{
    vec3 pos;
    vec3 rot;
    float scale;
    int type;
};

uniform float width;
uniform float height;

uniform int maxRays;
uniform float minDisRay;
uniform float maxDisRay;

uniform float colorSenstivity;
uniform float colorOffset;
uniform bool disFromZERO;

uniform vec3 colorA;
uniform vec3 colorB;
uniform vec3 colorC;
uniform vec3 colorD;

uniform vec3 camareDir;
uniform float camareRoll;
uniform vec3 camarePos;

uniform sampler2D background;
//uniform samplerCube skybox;
uniform vec3 backgroundcolor;
uniform float ContinuationOfRayColorOffset;
uniform float ContinuationOfRayColorSenstivity;

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

float opUnion( float d1, float d2 )
{
    return min(d1,d2);
}

float opSubtraction( float d1, float d2 )
{
    return max(-d1,d2);
}

float opIntersection( float d1, float d2 )
{
    return max(d1,d2);
}

float opXor(float d1, float d2 )
{
    return max(min(d1,d2),-max(d1,d2));
}

float opSmoothUnion( float d1, float d2, float k )
{
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h);
}

float opSmoothSubtraction( float d1, float d2, float k )
{
    float h = clamp( 0.5 - 0.5*(d2+d1)/k, 0.0, 1.0 );
    return mix( d2, -d1, h ) + k*h*(1.0-h);
}

float opSmoothIntersection( float d1, float d2, float k )
{
    float h = clamp( 0.5 - 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) + k*h*(1.0-h);
}

vec3 opTwist(vec3 p,float k)
{
    
    float c = cos(k*p.y);
    float s = sin(k*p.y);
    mat2  m = mat2(c,-s,s,c);
    vec3  q = vec3(m*p.xz,p.y);
    return q;
}


vec3 opCheapBend( vec3 p ,float k)
{
    float c = cos(k*p.x);
    float s = sin(k*p.x);
    mat2  m = mat2(c,-s,s,c);
    vec3  q = vec3(m*p.xy,p.z);
    return q;
}

vec3 opRepetitionXYZ(vec3 p,vec3 s )
{
    vec3 q = p - s*round(p/s);
    return q;
}

vec3 opRepetitionX(vec3 p,vec3 s)
{
    vec3 q = p;
    q.x = p.x - s.x*round(p.x/s.x);
    return q;
}

vec3 opRepetitionY(vec3 p,vec3 s)
{
    vec3 q = p;
    q.y = p.y - s.y*round(p.y/s.y);
    return q;
}

vec3 opRepetitionZ(vec3 p,vec3 s)
{
    vec3 q = p;
    q.z = p.z - s.z*round(p.z/s.z);
    return q;
}

vec3 opRepetitionXY(vec3 p,vec3 s)
{
    vec3 q = p;
    q.xy = p.xy - s.xy*round(p.xy/s.xy);
    return q;
}

vec3 opRepetitionXZ(vec3 p,vec3 s )
{
    vec3 q = p;
    q.xz = p.xz - s.xz*round(p.xz/s.xz);
    return q;
}

vec3 opRepetitionYZ(vec3 p,vec3 s)
{
    vec3 q = p;
    q.yz = p.yz - s.yz*round(p.yz/s.yz);
    return q;
}

vec3 opLimitedRepetition(vec3 p,vec3 s,vec3 l )
{
    vec3 q = p - s*clamp(round(p/s),-l,l);
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

float sdCone( vec3 p, vec2 c, float h )
{
  vec2 q = h*vec2(c.x/c.y,-1.0);
    
  vec2 w = vec2( length(p.xz), p.y );
  vec2 a = w - q*clamp( dot(w,q)/dot(q,q), 0.0, 1.0 );
  vec2 b = w - q*vec2( clamp( w.x/q.x, 0.0, 1.0 ), 1.0 );
  float k = sign( q.y );
  float d = min(dot( a, a ),dot(b, b));
  float s = max( k*(w.x*q.y-w.y*q.x),k*(w.y-q.y)  );
  return sqrt(d)*sign(s);
}

float sdCylinder( vec3 p, float h, float r )
{
  vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
  return min(max(d.x,d.y),0.0) + length(max(d,0.0));
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
            dis = sdTorus(p,vec2(1.0,0.3));
        }
        else if(tran[i].type == 2){
            dis = sdSphere(p,1.0);
        }
        else if(tran[i].type == 3){
            dis = sdCone(p,vec2(0.2,0.5),1.0);
        }
        else if(tran[i].type == 4){
            dis = sdCylinder(p,1.0,1.0);
        }
        //#else if else if#
        len = min (dis * tran[i].scale,len);
    }

    return len;
}

vec3 palette( in float t )
{
    return colorA + colorB*cos( 6.28318*(colorC*t+colorD) );
}

mat2 rot2D(float a) {
    return mat2(cos(a), -sin(a), sin(a), cos(a));
}

mat3 createRotationMatrix(vec3 direction, vec3 up) {
    vec3 right = cross(up, direction);
    vec3 newUp = cross(direction, right);
    return mat3(normalize(right), normalize(newUp), normalize(-direction));
}

vec3 getRayDir() {

    vec2 coords = ((fUV * 2.0 - 1.0)* vec2(width,height))/height;

    vec3 rayDirection = normalize(vec3(rot2D(camareRoll) * vec2(coords),1.0));

    mat3 rotationMatrix = createRotationMatrix(normalize(camareDir), vec3(0,1,0));

    //rotationMatrix = inverse(rotationMatrix);
    vec3 rayDir = rotationMatrix * rayDirection;
 return normalize(rayDir);
}

void main()
{
    vec3 rayDir = getRayDir();
    float dis;
    float totalDis = 0.0;vec3 p = vec3(0);
    for (int i = 0; i < maxRays; i++) {
        p = camarePos + rayDir * totalDis;

        dis = map(p);

        totalDis += dis;

        if (dis < minDisRay || totalDis > maxDisRay) break;
        
    }

    if(totalDis < maxDisRay  && dis > minDisRay){
            if(disFromZERO){totalDis = length(p);}
        Color = vec4(palette(totalDis * colorSenstivity + colorOffset), 1.0);
    }
    else if (length(texture(background, vec2(fUV.x,-fUV.y)).rgb) != 0.0){
        Color = texture(background, vec2(fUV.x,fUV.y));
    }
    else if (ContinuationOfRayColorSenstivity != 0.0){
        if(disFromZERO){totalDis = length(p);}
        Color = vec4(palette(totalDis * ContinuationOfRayColorSenstivity + ContinuationOfRayColorOffset), 1.0);
    }
    else if (ContinuationOfRayColorOffset != 0.0){
        if(disFromZERO){totalDis = length(p);}
        Color = vec4(palette(totalDis * colorSenstivity + colorOffset), 1.0);
    }
    else{
        Color = vec4(backgroundcolor,1);
    }

}
