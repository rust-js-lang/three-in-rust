js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
#ifdef USE_DISPLACEMENTMAP

	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );

#endif
"#;
}
