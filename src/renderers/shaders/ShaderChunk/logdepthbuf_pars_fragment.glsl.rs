js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )

	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;

#endif
"#;
}
