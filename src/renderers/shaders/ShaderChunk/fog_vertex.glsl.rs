js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_FOG

	vFogDepth = - mvPosition.z;

#endif
"#;
}
