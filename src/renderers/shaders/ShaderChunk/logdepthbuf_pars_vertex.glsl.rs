js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_LOGARITHMIC_DEPTH_BUFFER

	varying float vFragDepth;
	varying float vIsPerspective;

#endif
"#;
}
