pub struct GlFeatureNames {
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_multi_draw")]
    pub webgl_multi_draw: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_compressed_texture_astc")]
    pub webgl_compressed_texture_astc: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_compressed_texture_etc")]
    pub webgl_compressed_texture_etc: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_compressed_texture_etc1")]
    pub webgl_compressed_texture_etc1: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_compressed_texture_pvrtc")]
    pub webgl_compressed_texture_pvrtc: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_compressed_texture_s3tc")]
    pub webgl_compressed_texture_s3tc: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "EXT_texture_compression_bptc")]
    pub ext_texture_compression_bptc: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "EXT_disjoint_timer_query_webgl2")]
    pub ext_disjoint_timer_query_webgl2: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "OVR_multiview2")]
    pub ovr_multiview2: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "OES_texture_float_linear")]
    pub oes_texture_float_linear: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "WEBGL_clip_cull_distance")]
    pub webgl_clip_cull_distance: &'static str,
}

#[cfg_attr(rust_js, rust_js::name = "GLFeatureName")]
pub static GL_FEATURE_NAME: GlFeatureNames = GlFeatureNames {
    webgl_multi_draw: "WEBGL_multi_draw",
    webgl_compressed_texture_astc: "texture-compression-astc",
    webgl_compressed_texture_etc: "texture-compression-etc2",
    webgl_compressed_texture_etc1: "texture-compression-etc1",
    webgl_compressed_texture_pvrtc: "texture-compression-pvrtc",
    webgl_compressed_texture_s3tc: "texture-compression-s3tc",
    ext_texture_compression_bptc: "texture-compression-bc",
    ext_disjoint_timer_query_webgl2: "timestamp-query",
    ovr_multiview2: "OVR_multiview2",
    oes_texture_float_linear: "float32-filterable",
    webgl_clip_cull_distance: "clip-distances",
};
