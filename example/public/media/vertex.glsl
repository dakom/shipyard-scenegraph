precision mediump float;

attribute vec2 a_vertex;

uniform vec2 u_size;
uniform mat4 u_camera;
uniform mat4 u_model;

void main() {
    mat4 modelViewProjection = u_camera * u_model; 

    gl_Position = modelViewProjection * (vec4(u_size,0,1) * vec4(a_vertex,0,1));
}
