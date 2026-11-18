js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_METALNESSMAP

	uniform sampler2D metalnessMap;

#endif
"#;
}
