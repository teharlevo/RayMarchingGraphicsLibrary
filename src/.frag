#version 330 core


uniform vec3 camareAngles;
uniform vec3 camarePos;

in vec2 fUV;

out vec4 Color;

float map(vec3 pos){

    return length(pos) - 1.0;
}

mat3 calcLookAtMatrix(vec3 origin, vec3 target, float roll) {
  vec3 rr = vec3(sin(roll), cos(roll), 0.0);
  vec3 ww = normalize(target - origin);
  vec3 uu = normalize(cross(ww, rr));
  vec3 vv = normalize(cross(uu, ww));

  return mat3(uu, vv, ww);
}

void main()
{
    vec2 coords = fUV * 2.0 -1.0f;
    vec3 rayDirBase = normalize(vec3(coords, 1));
    //frant.x = (float)Math.cos(Math.toRadians(angleY - 90)) * (float)Math.cos(Math.toRadians(angleX));
    //frant.y = (float)Math.sin(Math.toRadians(angleX));
    //frant.z = (float)Math.sin(Math.toRadians(angleY - 90)) * (float)Math.cos(Math.toRadians(angleX));

    //rayDir.x *= cos(camareAngles.y - 3.14/2) * cos(camareAngles.x);
    //rayDir.y *= sin(camareAngles.x);
    //rayDir.z *= sin(camareAngles.y - 3.14/2) * cos(camareAngles.x);
    //x' = x cos θ − y sin θ
    //y' = x sin θ + y cos θ
    vec3 rayDir = rayDirBase;
    rayDir.x = rayDirBase.x * cos(camareAngles.x) - rayDirBase.y *  sin(camareAngles.x);
    rayDir.y = rayDirBase.x * sin(camareAngles.x) + rayDirBase.y *  cos(camareAngles.x);
    rayDir.z = 1.0;
    rayDir = normalize(rayDir);
    //rayDir = normalize(rayDir);

    float totalDis = 0.0;
    for (int i = 0; i < 80; i++) {
        vec3 p = camarePos + rayDir * totalDis;

        float dis = map(p);

        totalDis += dis;

        if (dis < .001 || totalDis > 1000.) break;
    }

    Color = vec4(totalDis * 0.2,totalDis * 0.2, totalDis * 0.2, 1.0);
}