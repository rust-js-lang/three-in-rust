js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif
"#;
}
