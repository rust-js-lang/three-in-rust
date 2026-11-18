js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#ifdef DITHERING

	gl_FragColor.rgb = dithering( gl_FragColor.rgb );

#endif
"#;
}
