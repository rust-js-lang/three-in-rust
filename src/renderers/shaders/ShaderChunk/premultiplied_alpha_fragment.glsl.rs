js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef PREMULTIPLIED_ALPHA

	gl_FragColor.rgb *= gl_FragColor.a;

#endif
"#;
}
