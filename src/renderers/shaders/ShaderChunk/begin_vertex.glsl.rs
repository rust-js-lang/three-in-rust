js::export_default! {
    const GLSL: &str = /* glsl */ r#"
vec3 transformed = vec3( position );

#ifdef USE_ALPHAHASH

	vPosition = vec3( position );

#endif
"#;
}
