js::export_default! {
    const GLSL: &str = /* glsl */ r#"
#if NUM_CLIPPING_PLANES > 0

	varying vec3 vClipPosition;

	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];

#endif
"#;
}
