js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_ROUGHNESSMAP

	uniform sampler2D roughnessMap;

#endif
"#;
}
