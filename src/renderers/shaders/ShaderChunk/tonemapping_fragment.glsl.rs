js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#if defined( TONE_MAPPING )

	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );

#endif
"#;
}
