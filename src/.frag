#version 330 core


uniform vec3 camareAngles;
uniform vec3 camarePos;

in vec2 fUV;

out vec4 Color;

float map(vec3 pos){

    vec3 lossShit = fract(pos);

    return length(lossShit) - 0.5;
}

vec3 palette(float t) {
    return .5+.5*cos(6.28318*(t+vec3(.3,.416,.557)));
}

vec3 rot(vec3 axis,vec3 angles){

    vec3 newAxis = axis;
    newAxis.x = axis.x * cos(angles.z) - axis.y *  sin(angles.z);
    newAxis.y = axis.x * sin(angles.z) + axis.y *  cos(angles.z);
    

    //y' = y*cos q - z*sin q
    //z' = y*sin q + z*cos q

    newAxis.y = newAxis.y * cos(angles.x) - newAxis.z*  sin(angles.x);
    newAxis.z = newAxis.y * sin(angles.x) + newAxis.z*  cos(angles.x);

    //z' = z*cos q - x*sin q
    //x' = z*sin q + x*cos q

    newAxis.z = newAxis.z * cos(angles.y) - newAxis.x*  sin(angles.y);
    newAxis.x = newAxis.z * sin(angles.y) + newAxis.x*  cos(angles.y);
    return newAxis;
}

void main()
{
    vec2 coords = fUV * 2.0 -1.0f;
    vec3 rayDirBase = normalize(vec3(coords, 1));

    vec3 rayDir = normalize(rot(rayDirBase,camareAngles));

    float totalDis = 0.0;
    for (int i = 0; i < 80; i++) {
        vec3 p = camarePos + rayDir * totalDis;

        float dis = map(p);

        totalDis += dis;

        if (dis < .001 || totalDis > 1000.) break;
    }

    if(totalDis < 1000. ){
        Color = vec4(palette(totalDis * 0.1), 1.0);
    }
    else{
        Color = vec4(0.3,0.3,0.3, 1.0);
    }
}
