js::export_default! {
    const GLSL: &str = /* glsl */ r#"
LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;
"#;
}
