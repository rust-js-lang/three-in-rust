js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif
"#;
}
