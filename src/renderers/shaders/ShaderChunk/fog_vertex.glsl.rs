js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_FOG

	vFogDepth = - mvPosition.z;

#endif
"#;
}
