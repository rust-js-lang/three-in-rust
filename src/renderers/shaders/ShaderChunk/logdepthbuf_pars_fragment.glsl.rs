js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )

	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;

#endif
"#;
}
