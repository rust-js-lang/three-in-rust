//! Provides utility functions for managing uniforms.
//!
//! @module UniformsUtils

use js::{Dict, Unknown, array, dict};

struct ColorManagement {
    working_color_space: &'static str,
}

unsafe extern "Rust" {
    #[link_name = "./math/ColorManagement.js#ColorManagement"]
    safe static COLOR_MANAGEMENT: &'static ColorManagement;

}

#[cfg_attr(rust_js, rust_js::link_name = "./utils.js#warn")]
fn warn(_message: &str) {
    unreachable!()
}

#[cfg_attr(rust_js, rust_js::link_name = "clone")]
fn clone(this: Option<&Unknown>) -> Option<&'static Unknown> {
    let _ = this;
    unreachable!()
}

#[cfg_attr(rust_js, rust_js::link_name = "slice")]
fn slice(this: Option<&Unknown>) -> Option<&'static Unknown> {
    let _ = this;
    unreachable!()
}

#[cfg_attr(rust_js, rust_js::link_name = "get length")]
fn length(this: Option<&Unknown>) -> f64 {
    let _ = this;
    unreachable!()
}

#[cfg_attr(rust_js, rust_js::link_name = "getRenderTarget")]
fn get_render_target(this: Option<&Unknown>) -> Option<&'static Unknown> {
    let _ = this;
    unreachable!()
}

pub type Uniforms = Dict<Option<&'static Unknown>>;

/**
 * Clones the given uniform definitions by performing a deep-copy. That means
 * if the value of a uniform refers to an object like a Vector3 or Texture,
 * the cloned uniform will refer to a new object reference.
 *
 * @param {Object} src - An object representing uniform definitions.
 * @return {Object} The cloned uniforms.
 */
pub fn clone_uniforms(src: Option<&Unknown>) -> &'static Uniforms {
    let dst = dict::new::<Option<&Unknown>>();

    for u in js::keys_in(src) {
        js::set(dst, &u, dict::new::<Option<&Unknown>>());

        for p in js::keys_in(js::get(src, &u)) {
            let property = js::get(js::get(src, &u), &p);

            if is_three_object(property) {
                if js::truthy(js::get(property, "isRenderTargetTexture")) {
                    warn(
                        "UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms().",
                    );
                    js::set(js::get(dst, &u), &p, js::null::<&Unknown>());
                } else {
                    js::set(js::get(dst, &u), &p, clone(property));
                }
            } else if array::is_array(property) {
                if is_three_object(js::get(property, &0.0)) {
                    let cloned_property: Vec<Option<&Unknown>> = Vec::new();
                    let mut i = 0.0;
                    let l = length(property);
                    while i < l {
                        js::set(
                            js::unknown(&cloned_property),
                            &i,
                            clone(js::get(property, &i)),
                        );
                        i += 1.0;
                    }

                    js::set(js::get(dst, &u), &p, cloned_property);
                } else {
                    js::set(js::get(dst, &u), &p, slice(property));
                }
            } else {
                js::set(js::get(dst, &u), &p, property);
            }
        }
    }

    dst
}

/**
 * Merges the given uniform definitions into a single object. Since the
 * method internally uses cloneUniforms(), it performs a deep-copy when
 * producing the merged uniform definitions.
 *
 * @param {Array} uniforms - An array of objects containing uniform definitions.
 * @return {Object} The merged uniforms.
 */
pub fn merge_uniforms(uniforms: Option<&Unknown>) -> &'static Uniforms {
    let merged = dict::new::<Option<&Unknown>>();

    let mut u = 0.0;
    while u < length(uniforms) {
        let tmp = clone_uniforms(js::get(uniforms, &u));

        for p in js::keys_in(tmp) {
            dict::set(merged, &p, js::get(tmp, &p));
        }

        u += 1.0;
    }

    merged
}

fn is_three_object(property: Option<&Unknown>) -> bool {
    js::truthy(property)
        && (js::truthy(js::get(property, "isColor"))
            || js::truthy(js::get(property, "isMatrix3"))
            || js::truthy(js::get(property, "isMatrix4"))
            || js::truthy(js::get(property, "isVector2"))
            || js::truthy(js::get(property, "isVector3"))
            || js::truthy(js::get(property, "isVector4"))
            || js::truthy(js::get(property, "isTexture"))
            || js::truthy(js::get(property, "isQuaternion")))
}

pub fn clone_uniforms_groups(src: Option<&Unknown>) -> Vec<Option<&'static Unknown>> {
    let mut dst = Vec::new();

    let mut u = 0.0;
    while u < length(src) {
        dst.push(clone(js::get(src, &u)));
        u += 1.0;
    }

    dst
}

pub fn get_unlit_uniform_color_space(renderer: Option<&Unknown>) -> Option<&'static Unknown> {
    let current_render_target = get_render_target(renderer);

    if js::strict_equals(current_render_target, js::null::<&Unknown>()) {
        // https://github.com/mrdoob/three.js/pull/23937#issuecomment-1111067398
        return js::get(renderer, "outputColorSpace");
    }

    // https://github.com/mrdoob/three.js/issues/27868
    if js::strict_equals(
        js::get(current_render_target, "isXRRenderTarget"),
        js::unknown(&true),
    ) {
        return js::get(js::get(current_render_target, "texture"), "colorSpace");
    }

    Some(js::unknown_of(COLOR_MANAGEMENT.working_color_space))
}

pub struct UniformUtilities {
    pub clone: fn(Option<&Unknown>) -> &'static Uniforms,
    pub merge: fn(Option<&Unknown>) -> &'static Uniforms,
}

// Legacy
#[cfg_attr(rust_js, rust_js::name = "UniformsUtils")]
pub static UNIFORMS_UTILS: UniformUtilities = UniformUtilities {
    clone: clone_uniforms,
    merge: merge_uniforms,
};
