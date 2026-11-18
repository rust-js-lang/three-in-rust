js::export_default! {
    const GLSL: &str = /* glsl */ r#"
gl_FragColor = linearToOutputTexel( gl_FragColor );
"#;
}
