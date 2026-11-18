js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#if NUM_CLIPPING_PLANES > 0

	varying vec3 vClipPosition;

#endif
"#;
}
