js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_EMISSIVEMAP

	uniform sampler2D emissiveMap;

#endif
"#;
}
