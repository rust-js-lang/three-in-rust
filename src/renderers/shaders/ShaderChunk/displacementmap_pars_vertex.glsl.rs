js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_DISPLACEMENTMAP

	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;

#endif
"#;
}
