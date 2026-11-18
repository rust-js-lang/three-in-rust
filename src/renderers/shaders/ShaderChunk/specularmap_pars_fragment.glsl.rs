js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_SPECULARMAP

	uniform sampler2D specularMap;

#endif
"#;
}
