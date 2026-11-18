js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_ALPHAMAP

	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;

#endif
"#;
}
