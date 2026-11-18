js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif
"#;
}
