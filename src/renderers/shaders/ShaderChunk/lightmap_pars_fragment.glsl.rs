js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_LIGHTMAP

	uniform sampler2D lightMap;
	uniform float lightMapIntensity;

#endif
"#;
}
