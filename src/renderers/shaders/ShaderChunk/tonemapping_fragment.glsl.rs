js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#if defined( TONE_MAPPING )

	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );

#endif
"#;
}
