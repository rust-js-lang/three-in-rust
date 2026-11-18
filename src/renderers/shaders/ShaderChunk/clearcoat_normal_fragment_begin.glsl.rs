js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_CLEARCOAT

	vec3 clearcoatNormal = nonPerturbedNormal;

#endif
"#;
}
