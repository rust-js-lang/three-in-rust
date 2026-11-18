js::export_default! {
    const GLSL: &str = /* glsl */ r#"
void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}
"#;
}
