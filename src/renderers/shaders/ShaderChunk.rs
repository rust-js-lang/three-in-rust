#[cfg_attr(rust_js, rust_js::name = "alphahash_fragment")]
use crate::shader_chunks::alphahash_fragment::GLSL as alphahash_fragment;
#[cfg_attr(rust_js, rust_js::name = "alphahash_pars_fragment")]
use crate::shader_chunks::alphahash_pars_fragment::GLSL as alphahash_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "alphamap_fragment")]
use crate::shader_chunks::alphamap_fragment::GLSL as alphamap_fragment;
#[cfg_attr(rust_js, rust_js::name = "alphamap_pars_fragment")]
use crate::shader_chunks::alphamap_pars_fragment::GLSL as alphamap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "alphatest_fragment")]
use crate::shader_chunks::alphatest_fragment::GLSL as alphatest_fragment;
#[cfg_attr(rust_js, rust_js::name = "alphatest_pars_fragment")]
use crate::shader_chunks::alphatest_pars_fragment::GLSL as alphatest_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "aomap_fragment")]
use crate::shader_chunks::aomap_fragment::GLSL as aomap_fragment;
#[cfg_attr(rust_js, rust_js::name = "aomap_pars_fragment")]
use crate::shader_chunks::aomap_pars_fragment::GLSL as aomap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "batching_pars_vertex")]
use crate::shader_chunks::batching_pars_vertex::GLSL as batching_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "batching_vertex")]
use crate::shader_chunks::batching_vertex::GLSL as batching_vertex;
#[cfg_attr(rust_js, rust_js::name = "begin_vertex")]
use crate::shader_chunks::begin_vertex::GLSL as begin_vertex;
#[cfg_attr(rust_js, rust_js::name = "beginnormal_vertex")]
use crate::shader_chunks::beginnormal_vertex::GLSL as beginnormal_vertex;
#[cfg_attr(rust_js, rust_js::name = "bsdfs")]
use crate::shader_chunks::bsdfs::GLSL as bsdfs;
#[cfg_attr(rust_js, rust_js::name = "iridescence_fragment")]
use crate::shader_chunks::iridescence_fragment::GLSL as iridescence_fragment;

#[cfg_attr(rust_js, rust_js::name = "bumpmap_pars_fragment")]
use crate::shader_chunks::bumpmap_pars_fragment::GLSL as bumpmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "clipping_planes_fragment")]
use crate::shader_chunks::clipping_planes_fragment::GLSL as clipping_planes_fragment;
#[cfg_attr(rust_js, rust_js::name = "clipping_planes_pars_fragment")]
use crate::shader_chunks::clipping_planes_pars_fragment::GLSL as clipping_planes_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "clipping_planes_pars_vertex")]
use crate::shader_chunks::clipping_planes_pars_vertex::GLSL as clipping_planes_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "clipping_planes_vertex")]
use crate::shader_chunks::clipping_planes_vertex::GLSL as clipping_planes_vertex;
#[cfg_attr(rust_js, rust_js::name = "color_fragment")]
use crate::shader_chunks::color_fragment::GLSL as color_fragment;
#[cfg_attr(rust_js, rust_js::name = "color_pars_fragment")]
use crate::shader_chunks::color_pars_fragment::GLSL as color_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "color_pars_vertex")]
use crate::shader_chunks::color_pars_vertex::GLSL as color_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "color_vertex")]
use crate::shader_chunks::color_vertex::GLSL as color_vertex;
#[cfg_attr(rust_js, rust_js::name = "common")]
use crate::shader_chunks::common::GLSL as common;
#[cfg_attr(rust_js, rust_js::name = "defaultnormal_vertex")]
use crate::shader_chunks::defaultnormal_vertex::GLSL as defaultnormal_vertex;
#[cfg_attr(rust_js, rust_js::name = "displacementmap_pars_vertex")]
use crate::shader_chunks::displacementmap_pars_vertex::GLSL as displacementmap_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "displacementmap_vertex")]
use crate::shader_chunks::displacementmap_vertex::GLSL as displacementmap_vertex;
#[cfg_attr(rust_js, rust_js::name = "emissivemap_fragment")]
use crate::shader_chunks::emissivemap_fragment::GLSL as emissivemap_fragment;
#[cfg_attr(rust_js, rust_js::name = "emissivemap_pars_fragment")]
use crate::shader_chunks::emissivemap_pars_fragment::GLSL as emissivemap_pars_fragment;

#[cfg_attr(rust_js, rust_js::name = "colorspace_fragment")]
use crate::shader_chunks::colorspace_fragment::GLSL as colorspace_fragment;
#[cfg_attr(rust_js, rust_js::name = "colorspace_pars_fragment")]
use crate::shader_chunks::colorspace_pars_fragment::GLSL as colorspace_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "envmap_fragment")]
use crate::shader_chunks::envmap_fragment::GLSL as envmap_fragment;

#[cfg_attr(rust_js, rust_js::name = "envmap_common_pars_fragment")]
use crate::shader_chunks::envmap_common_pars_fragment::GLSL as envmap_common_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "envmap_pars_fragment")]
use crate::shader_chunks::envmap_pars_fragment::GLSL as envmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "envmap_pars_vertex")]
use crate::shader_chunks::envmap_pars_vertex::GLSL as envmap_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "envmap_vertex")]
use crate::shader_chunks::envmap_vertex::GLSL as envmap_vertex;
#[cfg_attr(rust_js, rust_js::name = "fog_vertex")]
use crate::shader_chunks::fog_vertex::GLSL as fog_vertex;

#[cfg_attr(rust_js, rust_js::name = "fog_pars_vertex")]
use crate::shader_chunks::fog_pars_vertex::GLSL as fog_pars_vertex;

#[cfg_attr(rust_js, rust_js::name = "fog_fragment")]
use crate::shader_chunks::fog_fragment::GLSL as fog_fragment;
#[cfg_attr(rust_js, rust_js::name = "fog_pars_fragment")]
use crate::shader_chunks::fog_pars_fragment::GLSL as fog_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "gradientmap_pars_fragment")]
use crate::shader_chunks::gradientmap_pars_fragment::GLSL as gradientmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "lightmap_pars_fragment")]
use crate::shader_chunks::lightmap_pars_fragment::GLSL as lightmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_lambert_fragment")]
use crate::shader_chunks::lights_lambert_fragment::GLSL as lights_lambert_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_lambert_pars_fragment")]
use crate::shader_chunks::lights_lambert_pars_fragment::GLSL as lights_lambert_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_pars_begin")]
use crate::shader_chunks::lights_pars_begin::GLSL as lights_pars_begin;

#[cfg_attr(rust_js, rust_js::name = "envmap_physical_pars_fragment")]
use crate::shader_chunks::envmap_physical_pars_fragment::GLSL as envmap_physical_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_toon_fragment")]
use crate::shader_chunks::lights_toon_fragment::GLSL as lights_toon_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_toon_pars_fragment")]
use crate::shader_chunks::lights_toon_pars_fragment::GLSL as lights_toon_pars_fragment;

#[cfg_attr(rust_js, rust_js::name = "lights_phong_fragment")]
use crate::shader_chunks::lights_phong_fragment::GLSL as lights_phong_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_phong_pars_fragment")]
use crate::shader_chunks::lights_phong_pars_fragment::GLSL as lights_phong_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_physical_fragment")]
use crate::shader_chunks::lights_physical_fragment::GLSL as lights_physical_fragment;
#[cfg_attr(rust_js, rust_js::name = "lights_physical_pars_fragment")]
use crate::shader_chunks::lights_physical_pars_fragment::GLSL as lights_physical_pars_fragment;

#[cfg_attr(rust_js, rust_js::name = "lights_fragment_begin")]
use crate::shader_chunks::lights_fragment_begin::GLSL as lights_fragment_begin;
#[cfg_attr(rust_js, rust_js::name = "lights_fragment_maps")]
use crate::shader_chunks::lights_fragment_maps::GLSL as lights_fragment_maps;

#[cfg_attr(rust_js, rust_js::name = "lights_fragment_end")]
use crate::shader_chunks::lights_fragment_end::GLSL as lights_fragment_end;

#[cfg_attr(rust_js, rust_js::name = "lightprobes_pars_fragment")]
use crate::shader_chunks::lightprobes_pars_fragment::GLSL as lightprobes_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "logdepthbuf_fragment")]
use crate::shader_chunks::logdepthbuf_fragment::GLSL as logdepthbuf_fragment;
#[cfg_attr(rust_js, rust_js::name = "logdepthbuf_pars_fragment")]
use crate::shader_chunks::logdepthbuf_pars_fragment::GLSL as logdepthbuf_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "logdepthbuf_pars_vertex")]
use crate::shader_chunks::logdepthbuf_pars_vertex::GLSL as logdepthbuf_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "logdepthbuf_vertex")]
use crate::shader_chunks::logdepthbuf_vertex::GLSL as logdepthbuf_vertex;
#[cfg_attr(rust_js, rust_js::name = "map_fragment")]
use crate::shader_chunks::map_fragment::GLSL as map_fragment;
#[cfg_attr(rust_js, rust_js::name = "map_pars_fragment")]
use crate::shader_chunks::map_pars_fragment::GLSL as map_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "map_particle_fragment")]
use crate::shader_chunks::map_particle_fragment::GLSL as map_particle_fragment;
#[cfg_attr(rust_js, rust_js::name = "map_particle_pars_fragment")]
use crate::shader_chunks::map_particle_pars_fragment::GLSL as map_particle_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "metalnessmap_fragment")]
use crate::shader_chunks::metalnessmap_fragment::GLSL as metalnessmap_fragment;
#[cfg_attr(rust_js, rust_js::name = "metalnessmap_pars_fragment")]
use crate::shader_chunks::metalnessmap_pars_fragment::GLSL as metalnessmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "morphinstance_vertex")]
use crate::shader_chunks::morphinstance_vertex::GLSL as morphinstance_vertex;

#[cfg_attr(rust_js, rust_js::name = "morphcolor_vertex")]
use crate::shader_chunks::morphcolor_vertex::GLSL as morphcolor_vertex;
#[cfg_attr(rust_js, rust_js::name = "morphnormal_vertex")]
use crate::shader_chunks::morphnormal_vertex::GLSL as morphnormal_vertex;
#[cfg_attr(rust_js, rust_js::name = "morphtarget_pars_vertex")]
use crate::shader_chunks::morphtarget_pars_vertex::GLSL as morphtarget_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "morphtarget_vertex")]
use crate::shader_chunks::morphtarget_vertex::GLSL as morphtarget_vertex;
#[cfg_attr(rust_js, rust_js::name = "normal_fragment_begin")]
use crate::shader_chunks::normal_fragment_begin::GLSL as normal_fragment_begin;
#[cfg_attr(rust_js, rust_js::name = "normal_fragment_maps")]
use crate::shader_chunks::normal_fragment_maps::GLSL as normal_fragment_maps;
#[cfg_attr(rust_js, rust_js::name = "normal_pars_fragment")]
use crate::shader_chunks::normal_pars_fragment::GLSL as normal_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "normal_pars_vertex")]
use crate::shader_chunks::normal_pars_vertex::GLSL as normal_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "normal_vertex")]
use crate::shader_chunks::normal_vertex::GLSL as normal_vertex;
#[cfg_attr(rust_js, rust_js::name = "normalmap_pars_fragment")]
use crate::shader_chunks::normalmap_pars_fragment::GLSL as normalmap_pars_fragment;

#[cfg_attr(rust_js, rust_js::name = "clearcoat_normal_fragment_begin")]
use crate::shader_chunks::clearcoat_normal_fragment_begin::GLSL as clearcoat_normal_fragment_begin;
#[cfg_attr(rust_js, rust_js::name = "clearcoat_normal_fragment_maps")]
use crate::shader_chunks::clearcoat_normal_fragment_maps::GLSL as clearcoat_normal_fragment_maps;
#[cfg_attr(rust_js, rust_js::name = "clearcoat_pars_fragment")]
use crate::shader_chunks::clearcoat_pars_fragment::GLSL as clearcoat_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "iridescence_pars_fragment")]
use crate::shader_chunks::iridescence_pars_fragment::GLSL as iridescence_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "opaque_fragment")]
use crate::shader_chunks::opaque_fragment::GLSL as opaque_fragment;
#[cfg_attr(rust_js, rust_js::name = "packing")]
use crate::shader_chunks::packing::GLSL as packing;
#[cfg_attr(rust_js, rust_js::name = "premultiplied_alpha_fragment")]
use crate::shader_chunks::premultiplied_alpha_fragment::GLSL as premultiplied_alpha_fragment;
#[cfg_attr(rust_js, rust_js::name = "project_vertex")]
use crate::shader_chunks::project_vertex::GLSL as project_vertex;

#[cfg_attr(rust_js, rust_js::name = "dithering_fragment")]
use crate::shader_chunks::dithering_fragment::GLSL as dithering_fragment;
#[cfg_attr(rust_js, rust_js::name = "dithering_pars_fragment")]
use crate::shader_chunks::dithering_pars_fragment::GLSL as dithering_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "roughnessmap_fragment")]
use crate::shader_chunks::roughnessmap_fragment::GLSL as roughnessmap_fragment;
#[cfg_attr(rust_js, rust_js::name = "roughnessmap_pars_fragment")]
use crate::shader_chunks::roughnessmap_pars_fragment::GLSL as roughnessmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "shadowmap_pars_fragment")]
use crate::shader_chunks::shadowmap_pars_fragment::GLSL as shadowmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "shadowmap_pars_vertex")]
use crate::shader_chunks::shadowmap_pars_vertex::GLSL as shadowmap_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "shadowmap_vertex")]
use crate::shader_chunks::shadowmap_vertex::GLSL as shadowmap_vertex;
#[cfg_attr(rust_js, rust_js::name = "shadowmask_pars_fragment")]
use crate::shader_chunks::shadowmask_pars_fragment::GLSL as shadowmask_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "skinbase_vertex")]
use crate::shader_chunks::skinbase_vertex::GLSL as skinbase_vertex;
#[cfg_attr(rust_js, rust_js::name = "skinning_pars_vertex")]
use crate::shader_chunks::skinning_pars_vertex::GLSL as skinning_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "skinning_vertex")]
use crate::shader_chunks::skinning_vertex::GLSL as skinning_vertex;
#[cfg_attr(rust_js, rust_js::name = "skinnormal_vertex")]
use crate::shader_chunks::skinnormal_vertex::GLSL as skinnormal_vertex;
#[cfg_attr(rust_js, rust_js::name = "specularmap_fragment")]
use crate::shader_chunks::specularmap_fragment::GLSL as specularmap_fragment;
#[cfg_attr(rust_js, rust_js::name = "specularmap_pars_fragment")]
use crate::shader_chunks::specularmap_pars_fragment::GLSL as specularmap_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "tonemapping_fragment")]
use crate::shader_chunks::tonemapping_fragment::GLSL as tonemapping_fragment;
#[cfg_attr(rust_js, rust_js::name = "tonemapping_pars_fragment")]
use crate::shader_chunks::tonemapping_pars_fragment::GLSL as tonemapping_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "transmission_fragment")]
use crate::shader_chunks::transmission_fragment::GLSL as transmission_fragment;
#[cfg_attr(rust_js, rust_js::name = "transmission_pars_fragment")]
use crate::shader_chunks::transmission_pars_fragment::GLSL as transmission_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "uv_pars_fragment")]
use crate::shader_chunks::uv_pars_fragment::GLSL as uv_pars_fragment;
#[cfg_attr(rust_js, rust_js::name = "uv_pars_vertex")]
use crate::shader_chunks::uv_pars_vertex::GLSL as uv_pars_vertex;
#[cfg_attr(rust_js, rust_js::name = "uv_vertex")]
use crate::shader_chunks::uv_vertex::GLSL as uv_vertex;
#[cfg_attr(rust_js, rust_js::name = "worldpos_vertex")]
use crate::shader_chunks::worldpos_vertex::GLSL as worldpos_vertex;

struct ShaderProgram {
    vertex: &'static str,
    fragment: &'static str,
}

unsafe extern "Rust" {
    #[link_name = "./renderers/shaders/ShaderLib/background.glsl.js#* as background"]
    safe static BACKGROUND: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/backgroundCube.glsl.js#* as backgroundCube"]
    safe static BACKGROUND_CUBE: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/cube.glsl.js#* as cube"]
    safe static CUBE: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/depth.glsl.js#* as depth"]
    safe static DEPTH: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/distance.glsl.js#* as distance"]
    safe static DISTANCE: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/equirect.glsl.js#* as equirect"]
    safe static EQUIRECT: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/linedashed.glsl.js#* as linedashed"]
    safe static LINEDASHED: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshbasic.glsl.js#* as meshbasic"]
    safe static MESHBASIC: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshlambert.glsl.js#* as meshlambert"]
    safe static MESHLAMBERT: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshmatcap.glsl.js#* as meshmatcap"]
    safe static MESHMATCAP: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshnormal.glsl.js#* as meshnormal"]
    safe static MESHNORMAL: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshphong.glsl.js#* as meshphong"]
    safe static MESHPHONG: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshphysical.glsl.js#* as meshphysical"]
    safe static MESHPHYSICAL: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/meshtoon.glsl.js#* as meshtoon"]
    safe static MESHTOON: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/points.glsl.js#* as points"]
    safe static POINTS: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/shadow.glsl.js#* as shadow"]
    safe static SHADOW: &'static ShaderProgram;
    #[link_name = "./renderers/shaders/ShaderLib/sprite.glsl.js#* as sprite"]
    safe static SPRITE: &'static ShaderProgram;
}

pub struct ShaderChunks {
    #[cfg_attr(rust_js, rust_js::name = "alphahash_fragment")]
    pub alphahash_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "alphahash_pars_fragment")]
    pub alphahash_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "alphamap_fragment")]
    pub alphamap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "alphamap_pars_fragment")]
    pub alphamap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "alphatest_fragment")]
    pub alphatest_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "alphatest_pars_fragment")]
    pub alphatest_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "aomap_fragment")]
    pub aomap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "aomap_pars_fragment")]
    pub aomap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "batching_pars_vertex")]
    pub batching_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "batching_vertex")]
    pub batching_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "begin_vertex")]
    pub begin_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "beginnormal_vertex")]
    pub beginnormal_vertex: &'static str,
    pub bsdfs: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "iridescence_fragment")]
    pub iridescence_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "bumpmap_pars_fragment")]
    pub bumpmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clipping_planes_fragment")]
    pub clipping_planes_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clipping_planes_pars_fragment")]
    pub clipping_planes_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clipping_planes_pars_vertex")]
    pub clipping_planes_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clipping_planes_vertex")]
    pub clipping_planes_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "color_fragment")]
    pub color_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "color_pars_fragment")]
    pub color_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "color_pars_vertex")]
    pub color_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "color_vertex")]
    pub color_vertex: &'static str,
    pub common: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "defaultnormal_vertex")]
    pub defaultnormal_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "displacementmap_pars_vertex")]
    pub displacementmap_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "displacementmap_vertex")]
    pub displacementmap_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "emissivemap_fragment")]
    pub emissivemap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "emissivemap_pars_fragment")]
    pub emissivemap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "colorspace_fragment")]
    pub colorspace_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "colorspace_pars_fragment")]
    pub colorspace_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_fragment")]
    pub envmap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_common_pars_fragment")]
    pub envmap_common_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_pars_fragment")]
    pub envmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_pars_vertex")]
    pub envmap_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_physical_pars_fragment")]
    pub envmap_physical_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "envmap_vertex")]
    pub envmap_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "fog_vertex")]
    pub fog_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "fog_pars_vertex")]
    pub fog_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "fog_fragment")]
    pub fog_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "fog_pars_fragment")]
    pub fog_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "gradientmap_pars_fragment")]
    pub gradientmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lightmap_pars_fragment")]
    pub lightmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_lambert_fragment")]
    pub lights_lambert_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_lambert_pars_fragment")]
    pub lights_lambert_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_pars_begin")]
    pub lights_pars_begin: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_toon_fragment")]
    pub lights_toon_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_toon_pars_fragment")]
    pub lights_toon_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_phong_fragment")]
    pub lights_phong_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_phong_pars_fragment")]
    pub lights_phong_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_physical_fragment")]
    pub lights_physical_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_physical_pars_fragment")]
    pub lights_physical_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_fragment_begin")]
    pub lights_fragment_begin: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_fragment_maps")]
    pub lights_fragment_maps: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lights_fragment_end")]
    pub lights_fragment_end: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "lightprobes_pars_fragment")]
    pub lightprobes_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "logdepthbuf_fragment")]
    pub logdepthbuf_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "logdepthbuf_pars_fragment")]
    pub logdepthbuf_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "logdepthbuf_pars_vertex")]
    pub logdepthbuf_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "logdepthbuf_vertex")]
    pub logdepthbuf_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "map_fragment")]
    pub map_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "map_pars_fragment")]
    pub map_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "map_particle_fragment")]
    pub map_particle_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "map_particle_pars_fragment")]
    pub map_particle_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "metalnessmap_fragment")]
    pub metalnessmap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "metalnessmap_pars_fragment")]
    pub metalnessmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "morphinstance_vertex")]
    pub morphinstance_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "morphcolor_vertex")]
    pub morphcolor_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "morphnormal_vertex")]
    pub morphnormal_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "morphtarget_pars_vertex")]
    pub morphtarget_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "morphtarget_vertex")]
    pub morphtarget_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normal_fragment_begin")]
    pub normal_fragment_begin: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normal_fragment_maps")]
    pub normal_fragment_maps: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normal_pars_fragment")]
    pub normal_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normal_pars_vertex")]
    pub normal_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normal_vertex")]
    pub normal_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "normalmap_pars_fragment")]
    pub normalmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clearcoat_normal_fragment_begin")]
    pub clearcoat_normal_fragment_begin: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clearcoat_normal_fragment_maps")]
    pub clearcoat_normal_fragment_maps: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "clearcoat_pars_fragment")]
    pub clearcoat_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "iridescence_pars_fragment")]
    pub iridescence_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "opaque_fragment")]
    pub opaque_fragment: &'static str,
    pub packing: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "premultiplied_alpha_fragment")]
    pub premultiplied_alpha_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "project_vertex")]
    pub project_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "dithering_fragment")]
    pub dithering_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "dithering_pars_fragment")]
    pub dithering_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "roughnessmap_fragment")]
    pub roughnessmap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "roughnessmap_pars_fragment")]
    pub roughnessmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadowmap_pars_fragment")]
    pub shadowmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadowmap_pars_vertex")]
    pub shadowmap_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadowmap_vertex")]
    pub shadowmap_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadowmask_pars_fragment")]
    pub shadowmask_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "skinbase_vertex")]
    pub skinbase_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "skinning_pars_vertex")]
    pub skinning_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "skinning_vertex")]
    pub skinning_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "skinnormal_vertex")]
    pub skinnormal_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "specularmap_fragment")]
    pub specularmap_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "specularmap_pars_fragment")]
    pub specularmap_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "tonemapping_fragment")]
    pub tonemapping_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "tonemapping_pars_fragment")]
    pub tonemapping_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "transmission_fragment")]
    pub transmission_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "transmission_pars_fragment")]
    pub transmission_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "uv_pars_fragment")]
    pub uv_pars_fragment: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "uv_pars_vertex")]
    pub uv_pars_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "uv_vertex")]
    pub uv_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "worldpos_vertex")]
    pub worldpos_vertex: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "background_vert")]
    pub background_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "background_frag")]
    pub background_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "backgroundCube_vert")]
    pub background_cube_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "backgroundCube_frag")]
    pub background_cube_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "cube_vert")]
    pub cube_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "cube_frag")]
    pub cube_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "depth_vert")]
    pub depth_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "depth_frag")]
    pub depth_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "distance_vert")]
    pub distance_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "distance_frag")]
    pub distance_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "equirect_vert")]
    pub equirect_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "equirect_frag")]
    pub equirect_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "linedashed_vert")]
    pub linedashed_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "linedashed_frag")]
    pub linedashed_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshbasic_vert")]
    pub meshbasic_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshbasic_frag")]
    pub meshbasic_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshlambert_vert")]
    pub meshlambert_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshlambert_frag")]
    pub meshlambert_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshmatcap_vert")]
    pub meshmatcap_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshmatcap_frag")]
    pub meshmatcap_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshnormal_vert")]
    pub meshnormal_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshnormal_frag")]
    pub meshnormal_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshphong_vert")]
    pub meshphong_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshphong_frag")]
    pub meshphong_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshphysical_vert")]
    pub meshphysical_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshphysical_frag")]
    pub meshphysical_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshtoon_vert")]
    pub meshtoon_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "meshtoon_frag")]
    pub meshtoon_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "points_vert")]
    pub points_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "points_frag")]
    pub points_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadow_vert")]
    pub shadow_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "shadow_frag")]
    pub shadow_frag: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "sprite_vert")]
    pub sprite_vert: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "sprite_frag")]
    pub sprite_frag: &'static str,
}

thread_local! {
    #[cfg_attr(rust_js, rust_js::name = "ShaderChunk")]
    pub static SHADER_CHUNK: ShaderChunks = ShaderChunks {
        alphahash_fragment,
        alphahash_pars_fragment,
        alphamap_fragment,
        alphamap_pars_fragment,
        alphatest_fragment,
        alphatest_pars_fragment,
        aomap_fragment,
        aomap_pars_fragment,
        batching_pars_vertex,
        batching_vertex,
        begin_vertex,
        beginnormal_vertex,
        bsdfs,
        iridescence_fragment,
        bumpmap_pars_fragment,
        clipping_planes_fragment,
        clipping_planes_pars_fragment,
        clipping_planes_pars_vertex,
        clipping_planes_vertex,
        color_fragment,
        color_pars_fragment,
        color_pars_vertex,
        color_vertex,
        common,
        defaultnormal_vertex,
        displacementmap_pars_vertex,
        displacementmap_vertex,
        emissivemap_fragment,
        emissivemap_pars_fragment,
        colorspace_fragment,
        colorspace_pars_fragment,
        envmap_fragment,
        envmap_common_pars_fragment,
        envmap_pars_fragment,
        envmap_pars_vertex,
        envmap_physical_pars_fragment,
        envmap_vertex,
        fog_vertex,
        fog_pars_vertex,
        fog_fragment,
        fog_pars_fragment,
        gradientmap_pars_fragment,
        lightmap_pars_fragment,
        lights_lambert_fragment,
        lights_lambert_pars_fragment,
        lights_pars_begin,
        lights_toon_fragment,
        lights_toon_pars_fragment,
        lights_phong_fragment,
        lights_phong_pars_fragment,
        lights_physical_fragment,
        lights_physical_pars_fragment,
        lights_fragment_begin,
        lights_fragment_maps,
        lights_fragment_end,
        lightprobes_pars_fragment,
        logdepthbuf_fragment,
        logdepthbuf_pars_fragment,
        logdepthbuf_pars_vertex,
        logdepthbuf_vertex,
        map_fragment,
        map_pars_fragment,
        map_particle_fragment,
        map_particle_pars_fragment,
        metalnessmap_fragment,
        metalnessmap_pars_fragment,
        morphinstance_vertex,
        morphcolor_vertex,
        morphnormal_vertex,
        morphtarget_pars_vertex,
        morphtarget_vertex,
        normal_fragment_begin,
        normal_fragment_maps,
        normal_pars_fragment,
        normal_pars_vertex,
        normal_vertex,
        normalmap_pars_fragment,
        clearcoat_normal_fragment_begin,
        clearcoat_normal_fragment_maps,
        clearcoat_pars_fragment,
        iridescence_pars_fragment,
        opaque_fragment,
        packing,
        premultiplied_alpha_fragment,
        project_vertex,
        dithering_fragment,
        dithering_pars_fragment,
        roughnessmap_fragment,
        roughnessmap_pars_fragment,
        shadowmap_pars_fragment,
        shadowmap_pars_vertex,
        shadowmap_vertex,
        shadowmask_pars_fragment,
        skinbase_vertex,
        skinning_pars_vertex,
        skinning_vertex,
        skinnormal_vertex,
        specularmap_fragment,
        specularmap_pars_fragment,
        tonemapping_fragment,
        tonemapping_pars_fragment,
        transmission_fragment,
        transmission_pars_fragment,
        uv_pars_fragment,
        uv_pars_vertex,
        uv_vertex,
        worldpos_vertex,
        background_vert: BACKGROUND.vertex,
        background_frag: BACKGROUND.fragment,
        background_cube_vert: BACKGROUND_CUBE.vertex,
        background_cube_frag: BACKGROUND_CUBE.fragment,
        cube_vert: CUBE.vertex,
        cube_frag: CUBE.fragment,
        depth_vert: DEPTH.vertex,
        depth_frag: DEPTH.fragment,
        distance_vert: DISTANCE.vertex,
        distance_frag: DISTANCE.fragment,
        equirect_vert: EQUIRECT.vertex,
        equirect_frag: EQUIRECT.fragment,
        linedashed_vert: LINEDASHED.vertex,
        linedashed_frag: LINEDASHED.fragment,
        meshbasic_vert: MESHBASIC.vertex,
        meshbasic_frag: MESHBASIC.fragment,
        meshlambert_vert: MESHLAMBERT.vertex,
        meshlambert_frag: MESHLAMBERT.fragment,
        meshmatcap_vert: MESHMATCAP.vertex,
        meshmatcap_frag: MESHMATCAP.fragment,
        meshnormal_vert: MESHNORMAL.vertex,
        meshnormal_frag: MESHNORMAL.fragment,
        meshphong_vert: MESHPHONG.vertex,
        meshphong_frag: MESHPHONG.fragment,
        meshphysical_vert: MESHPHYSICAL.vertex,
        meshphysical_frag: MESHPHYSICAL.fragment,
        meshtoon_vert: MESHTOON.vertex,
        meshtoon_frag: MESHTOON.fragment,
        points_vert: POINTS.vertex,
        points_frag: POINTS.fragment,
        shadow_vert: SHADOW.vertex,
        shadow_frag: SHADOW.fragment,
        sprite_vert: SPRITE.vertex,
        sprite_frag: SPRITE.fragment,
    };
}
