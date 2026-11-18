js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )

	diffuseColor *= vColor;

#endif
"#;
}
