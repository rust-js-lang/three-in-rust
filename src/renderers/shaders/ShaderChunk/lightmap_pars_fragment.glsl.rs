js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_LIGHTMAP

	uniform sampler2D lightMap;
	uniform float lightMapIntensity;

#endif
"#;
}
