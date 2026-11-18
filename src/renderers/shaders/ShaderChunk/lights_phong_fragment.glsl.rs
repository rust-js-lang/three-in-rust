js::export_default! {
    const GLSL: &str = /* glsl */ r#"
BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;
"#;
}
