js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_ALPHAHASH

	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;

#endif
"#;
}
