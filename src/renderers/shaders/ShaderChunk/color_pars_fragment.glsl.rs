js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )

	varying vec4 vColor;

#endif
"#;
}
