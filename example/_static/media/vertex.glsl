precision mediump float;

attribute vec2 a_vertex;

varying vec2 v_uv;

uniform mat4 u_camera;
uniform mat4 u_model;

void main() {
    mat4 modelViewProjection = u_camera * u_model; 

    gl_Position = modelViewProjection * vec4(a_vertex,0,1);
    v_uv = a_vertex;
}
