js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#if NUM_CLIPPING_PLANES > 0

	vClipPosition = - mvPosition.xyz;

#endif
"#;
}
