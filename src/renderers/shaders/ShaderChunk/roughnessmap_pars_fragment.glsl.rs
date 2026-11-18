js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_ROUGHNESSMAP

	uniform sampler2D roughnessMap;

#endif
"#;
}
