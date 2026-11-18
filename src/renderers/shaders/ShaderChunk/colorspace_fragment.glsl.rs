js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
gl_FragColor = linearToOutputTexel( gl_FragColor );
"#;
}
