js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_ALPHAMAP

	uniform sampler2D alphaMap;

#endif
"#;
}
