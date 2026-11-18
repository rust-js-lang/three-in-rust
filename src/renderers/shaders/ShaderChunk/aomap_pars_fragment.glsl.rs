js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_AOMAP

	uniform sampler2D aoMap;
	uniform float aoMapIntensity;

#endif
"#;
}
