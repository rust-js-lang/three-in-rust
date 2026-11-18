js::export_default! {
    pub const GLSL: &str = /* glsl */ r#"
vec3 objectNormal = vec3( normal );

#ifdef USE_TANGENT

	vec3 objectTangent = vec3( tangent.xyz );

#endif
"#;
}
