/*!
 * This type represents mouse buttons and interaction types in context of controls.
 *
 * @typedef {Object} ConstantsMouse
 * @property {number} MIDDLE - The left mouse button.
 * @property {number} LEFT - The middle mouse button.
 * @property {number} RIGHT - The right mouse button.
 * @property {number} ROTATE - A rotate interaction.
 * @property {number} DOLLY - A dolly interaction.
 * @property {number} PAN - A pan interaction.
 **/

/*!
 * This type represents touch interaction types in context of controls.
 *
 * @typedef {Object} ConstantsTouch
 * @property {number} ROTATE - A rotate interaction.
 * @property {number} PAN - A pan interaction.
 * @property {number} DOLLY_PAN - The dolly-pan interaction.
 * @property {number} DOLLY_ROTATE - A dolly-rotate interaction.
 **/

/*!
 * This type represents the different timestamp query types.
 *
 * @typedef {Object} ConstantsTimestampQuery
 * @property {string} COMPUTE - A `compute` timestamp query.
 * @property {string} RENDER - A `render` timestamp query.
 **/

/*!
 * Represents the different interpolation sampling types.
 *
 * @typedef {Object} ConstantsInterpolationSamplingType
 * @property {string} PERSPECTIVE - Perspective-correct interpolation.
 * @property {string} LINEAR - Linear interpolation.
 * @property {string} FLAT - Flat interpolation.
 */

/*!
 * Represents the different interpolation sampling modes.
 *
 * @typedef {Object} ConstantsInterpolationSamplingMode
 * @property {string} NORMAL - Normal sampling mode.
 * @property {string} CENTROID - Centroid sampling mode.
 * @property {string} SAMPLE - Sample-specific sampling mode.
 * @property {string} FIRST - Flat interpolation using the first vertex.
 * @property {string} EITHER - Flat interpolation using either vertex.
 */

/*!
 * Represents the refresh types of render objects.
 *
 * @typedef {Object} ConstantsRenderObjectRefreshType
 * @property {number} NONE - No refresh required.
 * @property {number} SHARED - Only shared uniform buffers require an update.
 * @property {number} FULL - The render object requires a full refresh.
 */

pub struct ConstantsMouse {
    #[cfg_attr(rust_js, rust_js::name = "LEFT")]
    pub left: u32,
    #[cfg_attr(rust_js, rust_js::name = "MIDDLE")]
    pub middle: u32,
    #[cfg_attr(rust_js, rust_js::name = "RIGHT")]
    pub right: u32,
    #[cfg_attr(rust_js, rust_js::name = "ROTATE")]
    pub rotate: u32,
    #[cfg_attr(rust_js, rust_js::name = "DOLLY")]
    pub dolly: u32,
    #[cfg_attr(rust_js, rust_js::name = "PAN")]
    pub pan: u32,
}

pub struct ConstantsTouch {
    #[cfg_attr(rust_js, rust_js::name = "ROTATE")]
    pub rotate: u32,
    #[cfg_attr(rust_js, rust_js::name = "PAN")]
    pub pan: u32,
    #[cfg_attr(rust_js, rust_js::name = "DOLLY_PAN")]
    pub dolly_pan: u32,
    #[cfg_attr(rust_js, rust_js::name = "DOLLY_ROTATE")]
    pub dolly_rotate: u32,
}

pub struct ConstantsTimestampQuery {
    #[cfg_attr(rust_js, rust_js::name = "COMPUTE")]
    pub compute: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "RENDER")]
    pub render: &'static str,
}

pub struct ConstantsInterpolationSamplingType {
    #[cfg_attr(rust_js, rust_js::name = "PERSPECTIVE")]
    pub perspective: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "LINEAR")]
    pub linear: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "FLAT")]
    pub flat: &'static str,
}

pub struct ConstantsInterpolationSamplingMode {
    #[cfg_attr(rust_js, rust_js::name = "NORMAL")]
    pub normal: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "CENTROID")]
    pub centroid: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "SAMPLE")]
    pub sample: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "FIRST")]
    pub first: &'static str,
    #[cfg_attr(rust_js, rust_js::name = "EITHER")]
    pub either: &'static str,
}

pub struct CompatibilityFlags {
    #[cfg_attr(rust_js, rust_js::name = "TEXTURE_COMPARE")]
    pub texture_compare: &'static str,
}

pub struct ConstantsRenderObjectRefreshType {
    #[cfg_attr(rust_js, rust_js::name = "NONE")]
    pub none: u32,
    #[cfg_attr(rust_js, rust_js::name = "SHARED")]
    pub shared: u32,
    #[cfg_attr(rust_js, rust_js::name = "FULL")]
    pub full: u32,
}

pub const REVISION: &str = "188dev";

/**
 * Represents mouse buttons and interaction types in context of controls.
 *
 * @type {ConstantsMouse}
 * @constant
 */
pub static MOUSE: ConstantsMouse = ConstantsMouse {
    left: 0,
    middle: 1,
    right: 2,
    rotate: 0,
    dolly: 1,
    pan: 2,
};

/**
 * Represents touch interaction types in context of controls.
 *
 * @type {ConstantsTouch}
 * @constant
 */
pub static TOUCH: ConstantsTouch = ConstantsTouch {
    rotate: 0,
    pan: 1,
    dolly_pan: 2,
    dolly_rotate: 3,
};

/**
 * Disables face culling.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CullFaceNone")]
pub const CULL_FACE_NONE: u32 = 0;

/**
 * Culls back faces.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CullFaceBack")]
pub const CULL_FACE_BACK: u32 = 1;

/**
 * Culls front faces.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CullFaceFront")]
pub const CULL_FACE_FRONT: u32 = 2;

/**
 * Culls both front and back faces.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CullFaceFrontBack")]
pub const CULL_FACE_FRONT_BACK: u32 = 3;

/**
 * Gives unfiltered shadow maps - fastest, but lowest quality.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "BasicShadowMap")]
pub const BASIC_SHADOW_MAP: u32 = 0;

/**
 * Filters shadow maps using the Percentage-Closer Filtering (PCF) algorithm.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "PCFShadowMap")]
pub const PCF_SHADOW_MAP: u32 = 1;

/**
 * Filters shadow maps using the Percentage-Closer Filtering (PCF) algorithm with
 * better soft shadows especially when using low-resolution shadow maps.
 *
 * @type {number}
 * @constant
 * @deprecated since r186. Use `PCFShadowMap` instead.
 */
#[cfg_attr(rust_js, rust_js::name = "PCFSoftShadowMap")]
pub const PCF_SOFT_SHADOW_MAP: u32 = 2;

/**
 * Filters shadow maps using the Variance Shadow Map (VSM) algorithm.
 * When using VSMShadowMap all shadow receivers will also cast shadows.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "VSMShadowMap")]
pub const VSM_SHADOW_MAP: u32 = 3;

/**
 * Only front faces are rendered.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "FrontSide")]
pub const FRONT_SIDE: u32 = 0;

/**
 * Only back faces are rendered.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "BackSide")]
pub const BACK_SIDE: u32 = 1;

/**
 * Both front and back faces are rendered.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DoubleSide")]
pub const DOUBLE_SIDE: u32 = 2;

/**
 * No blending is performed which effectively disables
 * alpha transparency.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NoBlending")]
pub const NO_BLENDING: u32 = 0;

/**
 * The default blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NormalBlending")]
pub const NORMAL_BLENDING: u32 = 1;

/**
 * Represents additive blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AdditiveBlending")]
pub const ADDITIVE_BLENDING: u32 = 2;

/**
 * Represents subtractive blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SubtractiveBlending")]
pub const SUBTRACTIVE_BLENDING: u32 = 3;

/**
 * Represents multiply blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MultiplyBlending")]
pub const MULTIPLY_BLENDING: u32 = 4;

/**
 * Represents custom blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CustomBlending")]
pub const CUSTOM_BLENDING: u32 = 5;

/**
 * Represents material blending.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MaterialBlending")]
pub const MATERIAL_BLENDING: u32 = 6;

/**
 * A `source + destination` blending equation.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AddEquation")]
pub const ADD_EQUATION: u32 = 100;

/**
 * A `source - destination` blending equation.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SubtractEquation")]
pub const SUBTRACT_EQUATION: u32 = 101;

/**
 * A `destination - source` blending equation.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ReverseSubtractEquation")]
pub const REVERSE_SUBTRACT_EQUATION: u32 = 102;

/**
 * A blend equation that uses the minimum of source and destination.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MinEquation")]
pub const MIN_EQUATION: u32 = 103;

/**
 * A blend equation that uses the maximum of source and destination.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MaxEquation")]
pub const MAX_EQUATION: u32 = 104;

/**
 * Multiplies all colors by `0`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ZeroFactor")]
pub const ZERO_FACTOR: u32 = 200;

/**
 * Multiplies all colors by `1`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneFactor")]
pub const ONE_FACTOR: u32 = 201;

/**
 * Multiplies all colors by the source colors.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SrcColorFactor")]
pub const SRC_COLOR_FACTOR: u32 = 202;

/**
 * Multiplies all colors by `1` minus each source color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusSrcColorFactor")]
pub const ONE_MINUS_SRC_COLOR_FACTOR: u32 = 203;

/**
 * Multiplies all colors by the source alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SrcAlphaFactor")]
pub const SRC_ALPHA_FACTOR: u32 = 204;

/**
 * Multiplies all colors by 1 minus the source alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusSrcAlphaFactor")]
pub const ONE_MINUS_SRC_ALPHA_FACTOR: u32 = 205;

/**
 * Multiplies all colors by the destination alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DstAlphaFactor")]
pub const DST_ALPHA_FACTOR: u32 = 206;

/**
 * Multiplies all colors by `1` minus the destination alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusDstAlphaFactor")]
pub const ONE_MINUS_DST_ALPHA_FACTOR: u32 = 207;

/**
 * Multiplies all colors by the destination color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DstColorFactor")]
pub const DST_COLOR_FACTOR: u32 = 208;

/**
 * Multiplies all colors by `1` minus each destination color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusDstColorFactor")]
pub const ONE_MINUS_DST_COLOR_FACTOR: u32 = 209;

/**
 * Multiplies the RGB colors by the smaller of either the source alpha
 * value or the value of `1` minus the destination alpha value. The alpha
 * value is multiplied by `1`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SrcAlphaSaturateFactor")]
pub const SRC_ALPHA_SATURATE_FACTOR: u32 = 210;

/**
 * Multiplies all colors by a constant color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ConstantColorFactor")]
pub const CONSTANT_COLOR_FACTOR: u32 = 211;

/**
 * Multiplies all colors by `1` minus a constant color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusConstantColorFactor")]
pub const ONE_MINUS_CONSTANT_COLOR_FACTOR: u32 = 212;

/**
 * Multiplies all colors by a constant alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ConstantAlphaFactor")]
pub const CONSTANT_ALPHA_FACTOR: u32 = 213;

/**
 * Multiplies all colors by 1 minus a constant alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "OneMinusConstantAlphaFactor")]
pub const ONE_MINUS_CONSTANT_ALPHA_FACTOR: u32 = 214;

/**
 * Never pass.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NeverDepth")]
pub const NEVER_DEPTH: u32 = 0;

/**
 * Always pass.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AlwaysDepth")]
pub const ALWAYS_DEPTH: u32 = 1;

/**
 * Pass if the incoming value is less than the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessDepth")]
pub const LESS_DEPTH: u32 = 2;

/**
 * Pass if the incoming value is less than or equal to the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessEqualDepth")]
pub const LESS_EQUAL_DEPTH: u32 = 3;

/**
 * Pass if the incoming value equals the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "EqualDepth")]
pub const EQUAL_DEPTH: u32 = 4;

/**
 * Pass if the incoming value is greater than or equal to the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterEqualDepth")]
pub const GREATER_EQUAL_DEPTH: u32 = 5;

/**
 * Pass if the incoming value is greater than the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterDepth")]
pub const GREATER_DEPTH: u32 = 6;

/**
 * Pass if the incoming value is not equal to the depth buffer value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NotEqualDepth")]
pub const NOT_EQUAL_DEPTH: u32 = 7;

/**
 * Multiplies the environment map color with the surface color.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MultiplyOperation")]
pub const MULTIPLY_OPERATION: u32 = 0;

/**
 * Uses reflectivity to blend between the two colors.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MixOperation")]
pub const MIX_OPERATION: u32 = 1;

/**
 * Adds the two colors.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AddOperation")]
pub const ADD_OPERATION: u32 = 2;

/**
 * No tone mapping is applied.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NoToneMapping")]
pub const NO_TONE_MAPPING: u32 = 0;

/**
 * Linear tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearToneMapping")]
pub const LINEAR_TONE_MAPPING: u32 = 1;

/**
 * Reinhard tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ReinhardToneMapping")]
pub const REINHARD_TONE_MAPPING: u32 = 2;

/**
 * Cineon tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CineonToneMapping")]
pub const CINEON_TONE_MAPPING: u32 = 3;

/**
 * ACES Filmic tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ACESFilmicToneMapping")]
pub const ACES_FILMIC_TONE_MAPPING: u32 = 4;

/**
 * Custom tone mapping.
 *
 * Expects a custom implementation by modifying shader code of the material's fragment shader.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CustomToneMapping")]
pub const CUSTOM_TONE_MAPPING: u32 = 5;

/**
 * AgX tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AgXToneMapping")]
pub const AG_X_TONE_MAPPING: u32 = 6;

/**
 * Neutral tone mapping.
 *
 * Implementation based on the Khronos 3D Commerce Group standard tone mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NeutralToneMapping")]
pub const NEUTRAL_TONE_MAPPING: u32 = 7;

/**
 * The skinned mesh shares the same world space as the skeleton.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AttachedBindMode")]
pub const ATTACHED_BIND_MODE: &str = "attached";

/**
 * The skinned mesh does not share the same world space as the skeleton.
 * This is useful when a skeleton is shared across multiple skinned meshes.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DetachedBindMode")]
pub const DETACHED_BIND_MODE: &str = "detached";

/**
 * Maps textures using the geometry's UV coordinates.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UVMapping")]
pub const UV_MAPPING: u32 = 300;

/**
 * Reflection mapping for cube textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CubeReflectionMapping")]
pub const CUBE_REFLECTION_MAPPING: u32 = 301;

/**
 * Refraction mapping for cube textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "CubeRefractionMapping")]
pub const CUBE_REFRACTION_MAPPING: u32 = 302;

/**
 * Reflection mapping for equirectangular textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "EquirectangularReflectionMapping")]
pub const EQUIRECTANGULAR_REFLECTION_MAPPING: u32 = 303;

/**
 * Refraction mapping for equirectangular textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "EquirectangularRefractionMapping")]
pub const EQUIRECTANGULAR_REFRACTION_MAPPING: u32 = 304;

/**
 * The texture will simply repeat to infinity.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RepeatWrapping")]
pub const REPEAT_WRAPPING: u32 = 1000;

/**
 * The last pixel of the texture stretches to the edge of the mesh.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ClampToEdgeWrapping")]
pub const CLAMP_TO_EDGE_WRAPPING: u32 = 1001;

/**
 * The texture will repeats to infinity, mirroring on each repeat.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "MirroredRepeatWrapping")]
pub const MIRRORED_REPEAT_WRAPPING: u32 = 1002;

/**
 * Returns the value of the texture element that is nearest (in Manhattan distance)
 * to the specified texture coordinates.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NearestFilter")]
pub const NEAREST_FILTER: u32 = 1003;

/**
 * Chooses the mipmap that most closely matches the size of the pixel being textured
 * and uses the `NearestFilter` criterion (the texel nearest to the center of the pixel)
 * to produce a texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NearestMipmapNearestFilter")]
pub const NEAREST_MIPMAP_NEAREST_FILTER: u32 = 1004;
#[cfg_attr(rust_js, rust_js::name = "NearestMipMapNearestFilter")]
pub const NEAREST_MIP_MAP_NEAREST_FILTER: u32 = 1004; // legacy

/**
 * Chooses the two mipmaps that most closely match the size of the pixel being textured and
 * uses the `NearestFilter` criterion to produce a texture value from each mipmap.
 * The final texture value is a weighted average of those two values.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NearestMipmapLinearFilter")]
pub const NEAREST_MIPMAP_LINEAR_FILTER: u32 = 1005;
#[cfg_attr(rust_js, rust_js::name = "NearestMipMapLinearFilter")]
pub const NEAREST_MIP_MAP_LINEAR_FILTER: u32 = 1005; // legacy

/**
 * Returns the weighted average of the four texture elements that are closest to the specified
 * texture coordinates, and can include items wrapped or repeated from other parts of a texture,
 * depending on the values of `wrapS` and `wrapT`, and on the exact mapping.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearFilter")]
pub const LINEAR_FILTER: u32 = 1006;

/**
 * Chooses the mipmap that most closely matches the size of the pixel being textured and uses
 * the `LinearFilter` criterion (a weighted average of the four texels that are closest to the
 * center of the pixel) to produce a texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearMipmapNearestFilter")]
pub const LINEAR_MIPMAP_NEAREST_FILTER: u32 = 1007;
#[cfg_attr(rust_js, rust_js::name = "LinearMipMapNearestFilter")]
pub const LINEAR_MIP_MAP_NEAREST_FILTER: u32 = 1007; // legacy

/**
 * Chooses the two mipmaps that most closely match the size of the pixel being textured and uses
 * the `LinearFilter` criterion to produce a texture value from each mipmap. The final texture value
 * is a weighted average of those two values.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearMipmapLinearFilter")]
pub const LINEAR_MIPMAP_LINEAR_FILTER: u32 = 1008;
#[cfg_attr(rust_js, rust_js::name = "LinearMipMapLinearFilter")]
pub const LINEAR_MIP_MAP_LINEAR_FILTER: u32 = 1008; // legacy

/**
 * An unsigned byte data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedByteType")]
pub const UNSIGNED_BYTE_TYPE: u32 = 1009;

/**
 * A byte data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ByteType")]
pub const BYTE_TYPE: u32 = 1010;

/**
 * A short data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ShortType")]
pub const SHORT_TYPE: u32 = 1011;

/**
 * An unsigned short data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedShortType")]
pub const UNSIGNED_SHORT_TYPE: u32 = 1012;

/**
 * An int data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "IntType")]
pub const INT_TYPE: u32 = 1013;

/**
 * An unsigned int data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedIntType")]
pub const UNSIGNED_INT_TYPE: u32 = 1014;

/**
 * A float data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "FloatType")]
pub const FLOAT_TYPE: u32 = 1015;

/**
 * A half float data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "HalfFloatType")]
pub const HALF_FLOAT_TYPE: u32 = 1016;

/**
 * An unsigned short 4_4_4_4 (packed) data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedShort4444Type")]
pub const UNSIGNED_SHORT4444_TYPE: u32 = 1017;

/**
 * An unsigned short 5_5_5_1 (packed) data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedShort5551Type")]
pub const UNSIGNED_SHORT5551_TYPE: u32 = 1018;

/**
 * An unsigned int 24_8 data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedInt248Type")]
pub const UNSIGNED_INT248_TYPE: u32 = 1020;

/**
 * An unsigned int 5_9_9_9 (packed) data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedInt5999Type")]
pub const UNSIGNED_INT5999_TYPE: u32 = 35902;

/**
 * An unsigned int 10_11_11 (packed) data type for textures.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "UnsignedInt101111Type")]
pub const UNSIGNED_INT101111_TYPE: u32 = 35899;

/**
 * Discards the red, green and blue components and reads just the alpha component.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AlphaFormat")]
pub const ALPHA_FORMAT: u32 = 1021;

/**
 * Discards the alpha component and reads the red, green and blue component.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBFormat")]
pub const RGB_FORMAT: u32 = 1022;

/**
 * Reads the red, green, blue and alpha components.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBAFormat")]
pub const RGBA_FORMAT: u32 = 1023;

/**
 * Reads each element as a single depth value, converts it to floating point, and clamps to the range `[0,1]`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DepthFormat")]
pub const DEPTH_FORMAT: u32 = 1026;

/**
 * Reads each element is a pair of depth and stencil values. The depth component of the pair is interpreted as
 * in `DepthFormat`. The stencil component is interpreted based on the depth + stencil internal format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DepthStencilFormat")]
pub const DEPTH_STENCIL_FORMAT: u32 = 1027;

/**
 * Discards the green, blue and alpha components and reads just the red component.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RedFormat")]
pub const RED_FORMAT: u32 = 1028;

/**
 * Discards the green, blue and alpha components and reads just the red component. The texels are read as integers instead of floating point.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RedIntegerFormat")]
pub const RED_INTEGER_FORMAT: u32 = 1029;

/**
 * Discards the alpha, and blue components and reads the red, and green components.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGFormat")]
pub const RG_FORMAT: u32 = 1030;

/**
 * Discards the alpha, and blue components and reads the red, and green components. The texels are read as integers instead of floating point.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGIntegerFormat")]
pub const RG_INTEGER_FORMAT: u32 = 1031;

/**
 * Discards the alpha component and reads the red, green and blue component. The texels are read as integers instead of floating point.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBIntegerFormat")]
pub const RGB_INTEGER_FORMAT: u32 = 1032;

/**
 * Reads the red, green, blue and alpha components. The texels are read as integers instead of floating point.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBAIntegerFormat")]
pub const RGBA_INTEGER_FORMAT: u32 = 1033;

/**
 * A DXT1-compressed image in an RGB image format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_S3TC_DXT1_Format")]
pub const RGB_S3_TC_DXT1_FORMAT: u32 = 33776;

/**
 * A DXT1-compressed image in an RGB image format with a simple on/off alpha value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_S3TC_DXT1_Format")]
pub const RGBA_S3_TC_DXT1_FORMAT: u32 = 33777;

/**
 * A DXT3-compressed image in an RGBA image format. Compared to a 32-bit RGBA texture, it offers 4:1 compression.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_S3TC_DXT3_Format")]
pub const RGBA_S3_TC_DXT3_FORMAT: u32 = 33778;

/**
 * A DXT5-compressed image in an RGBA image format. It also provides a 4:1 compression, but differs to the DXT3
 * compression in how the alpha compression is done.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_S3TC_DXT5_Format")]
pub const RGBA_S3_TC_DXT5_FORMAT: u32 = 33779;

/**
 * PVRTC RGB compression in 4-bit mode. One block for each 4×4 pixels.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_PVRTC_4BPPV1_Format")]
pub const RGB_PVRTC_4_BPPV1_FORMAT: u32 = 35840;

/**
 * PVRTC RGB compression in 2-bit mode. One block for each 8×4 pixels.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_PVRTC_2BPPV1_Format")]
pub const RGB_PVRTC_2_BPPV1_FORMAT: u32 = 35841;

/**
 * PVRTC RGBA compression in 4-bit mode. One block for each 4×4 pixels.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_PVRTC_4BPPV1_Format")]
pub const RGBA_PVRTC_4_BPPV1_FORMAT: u32 = 35842;

/**
 * PVRTC RGBA compression in 2-bit mode. One block for each 8×4 pixels.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_PVRTC_2BPPV1_Format")]
pub const RGBA_PVRTC_2_BPPV1_FORMAT: u32 = 35843;

/**
 * ETC1 RGB format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_ETC1_Format")]
pub const RGB_ETC1_FORMAT: u32 = 36196;

/**
 * ETC2 RGB format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_ETC2_Format")]
pub const RGB_ETC2_FORMAT: u32 = 37492;

/**
 * ETC2 RGBA format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ETC2_EAC_Format")]
pub const RGBA_ETC2_EAC_FORMAT: u32 = 37496;

/**
 * EAC R11 UNORM format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "R11_EAC_Format")]
pub const R11_EAC_FORMAT: u32 = 37488; // 0x9270

/**
 * EAC R11 SNORM format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SIGNED_R11_EAC_Format")]
pub const SIGNED_R11_EAC_FORMAT: u32 = 37489; // 0x9271

/**
 * EAC RG11 UNORM format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RG11_EAC_Format")]
pub const RG11_EAC_FORMAT: u32 = 37490; // 0x9272

/**
 * EAC RG11 SNORM format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SIGNED_RG11_EAC_Format")]
pub const SIGNED_RG11_EAC_FORMAT: u32 = 37491; // 0x9273

/**
 * ASTC RGBA 4x4 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_4x4_Format")]
pub const RGBA_ASTC_4X4_FORMAT: u32 = 37808;

/**
 * ASTC RGBA 5x4 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_5x4_Format")]
pub const RGBA_ASTC_5X4_FORMAT: u32 = 37809;

/**
 * ASTC RGBA 5x5 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_5x5_Format")]
pub const RGBA_ASTC_5X5_FORMAT: u32 = 37810;

/**
 * ASTC RGBA 6x5 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_6x5_Format")]
pub const RGBA_ASTC_6X5_FORMAT: u32 = 37811;

/**
 * ASTC RGBA 6x6 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_6x6_Format")]
pub const RGBA_ASTC_6X6_FORMAT: u32 = 37812;

/**
 * ASTC RGBA 8x5 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_8x5_Format")]
pub const RGBA_ASTC_8X5_FORMAT: u32 = 37813;

/**
 * ASTC RGBA 8x6 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_8x6_Format")]
pub const RGBA_ASTC_8X6_FORMAT: u32 = 37814;

/**
 * ASTC RGBA 8x8 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_8x8_Format")]
pub const RGBA_ASTC_8X8_FORMAT: u32 = 37815;

/**
 * ASTC RGBA 10x5 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_10x5_Format")]
pub const RGBA_ASTC_10X5_FORMAT: u32 = 37816;

/**
 * ASTC RGBA 10x6 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_10x6_Format")]
pub const RGBA_ASTC_10X6_FORMAT: u32 = 37817;

/**
 * ASTC RGBA 10x8 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_10x8_Format")]
pub const RGBA_ASTC_10X8_FORMAT: u32 = 37818;

/**
 * ASTC RGBA 10x10 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_10x10_Format")]
pub const RGBA_ASTC_10X10_FORMAT: u32 = 37819;

/**
 * ASTC RGBA 12x10 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_12x10_Format")]
pub const RGBA_ASTC_12X10_FORMAT: u32 = 37820;

/**
 * ASTC RGBA 12x12 format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_ASTC_12x12_Format")]
pub const RGBA_ASTC_12X12_FORMAT: u32 = 37821;

/**
 * BPTC RGBA format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBA_BPTC_Format")]
pub const RGBA_BPTC_FORMAT: u32 = 36492;

/**
 * BPTC Signed RGB format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_BPTC_SIGNED_Format")]
pub const RGB_BPTC_SIGNED_FORMAT: u32 = 36494;

/**
 * BPTC Unsigned RGB format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGB_BPTC_UNSIGNED_Format")]
pub const RGB_BPTC_UNSIGNED_FORMAT: u32 = 36495;

/**
 * RGTC1 Red format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RED_RGTC1_Format")]
pub const RED_RGTC1_FORMAT: u32 = 36283;

/**
 * RGTC1 Signed Red format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SIGNED_RED_RGTC1_Format")]
pub const SIGNED_RED_RGTC1_FORMAT: u32 = 36284;

/**
 * RGTC2 Red Green format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RED_GREEN_RGTC2_Format")]
pub const RED_GREEN_RGTC2_FORMAT: u32 = 36285;

/**
 * RGTC2 Signed Red Green format.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SIGNED_RED_GREEN_RGTC2_Format")]
pub const SIGNED_RED_GREEN_RGTC2_FORMAT: u32 = 36286;

/**
 * Animations are played once.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LoopOnce")]
pub const LOOP_ONCE: u32 = 2200;

/**
 * Animations are played with a chosen number of repetitions, each time jumping from
 * the end of the clip directly to its beginning.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LoopRepeat")]
pub const LOOP_REPEAT: u32 = 2201;

/**
 * Animations are played with a chosen number of repetitions, alternately playing forward
 * and backward.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LoopPingPong")]
pub const LOOP_PING_PONG: u32 = 2202;

/**
 * Discrete interpolation mode for keyframe tracks.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolateDiscrete")]
pub const INTERPOLATE_DISCRETE: u32 = 2300;

/**
 * Linear interpolation mode for keyframe tracks.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolateLinear")]
pub const INTERPOLATE_LINEAR: u32 = 2301;

/**
 * Smooth interpolation mode for keyframe tracks.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolateSmooth")]
pub const INTERPOLATE_SMOOTH: u32 = 2302;

/**
 * Bezier interpolation mode for keyframe tracks.
 *
 * Uses cubic Bezier curves with explicit 2D control points.
 * Requires tangent data to be set on the track.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolateBezier")]
pub const INTERPOLATE_BEZIER: u32 = 2303;

/**
 * Zero curvature ending for animations.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ZeroCurvatureEnding")]
pub const ZERO_CURVATURE_ENDING: u32 = 2400;

/**
 * Zero slope ending for animations.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ZeroSlopeEnding")]
pub const ZERO_SLOPE_ENDING: u32 = 2401;

/**
 * Wrap around ending for animations.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "WrapAroundEnding")]
pub const WRAP_AROUND_ENDING: u32 = 2402;

/**
 * Default animation blend mode.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NormalAnimationBlendMode")]
pub const NORMAL_ANIMATION_BLEND_MODE: u32 = 2500;

/**
 * Additive animation blend mode. Can be used to layer motions on top of
 * each other to build complex performances from smaller re-usable assets.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AdditiveAnimationBlendMode")]
pub const ADDITIVE_ANIMATION_BLEND_MODE: u32 = 2501;

/**
 * For every three vertices draw a single triangle.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "TrianglesDrawMode")]
pub const TRIANGLES_DRAW_MODE: u32 = 0;

/**
 * For each vertex draw a triangle from the last three vertices.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "TriangleStripDrawMode")]
pub const TRIANGLE_STRIP_DRAW_MODE: u32 = 1;

/**
 * For each vertex draw a triangle from the first vertex and the last two vertices.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "TriangleFanDrawMode")]
pub const TRIANGLE_FAN_DRAW_MODE: u32 = 2;

/**
 * The depth value is inverted (1.0 - z) for visualization purposes.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "BasicDepthPacking")]
pub const BASIC_DEPTH_PACKING: u32 = 3200;

/**
 * The depth value is packed into 32 bit RGBA.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBADepthPacking")]
pub const RGBA_DEPTH_PACKING: u32 = 3201;

/**
 * The depth value is packed into 24 bit RGB.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGBDepthPacking")]
pub const RGB_DEPTH_PACKING: u32 = 3202;

/**
 * The depth value is packed into 16 bit RG.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RGDepthPacking")]
pub const RG_DEPTH_PACKING: u32 = 3203;

/**
 * Normal information is relative to the underlying surface.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "TangentSpaceNormalMap")]
pub const TANGENT_SPACE_NORMAL_MAP: u32 = 0;

/**
 * Normal information is relative to the object orientation.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ObjectSpaceNormalMap")]
pub const OBJECT_SPACE_NORMAL_MAP: u32 = 1;

// Color space string identifiers, matching CSS Color Module Level 4 and WebGPU names where available.

/**
 * No color space.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NoColorSpace")]
pub const NO_COLOR_SPACE: &str = "";

/**
 * sRGB color space.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SRGBColorSpace")]
pub const SRGB_COLOR_SPACE: &str = "srgb";

/**
 * sRGB-linear color space.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearSRGBColorSpace")]
pub const LINEAR_SRGB_COLOR_SPACE: &str = "srgb-linear";

/**
 * Linear transfer function.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LinearTransfer")]
pub const LINEAR_TRANSFER: &str = "linear";

/**
 * sRGB transfer function.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "SRGBTransfer")]
pub const SRGB_TRANSFER: &str = "srgb";

/**
 * No normal map packing.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NoNormalPacking")]
pub const NO_NORMAL_PACKING: &str = "";

/**
 * Normal RG packing.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NormalRGPacking")]
pub const NORMAL_RG_PACKING: &str = "rg";

/**
 * Normal GA packing.
 *
 * @type {string}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NormalGAPacking")]
pub const NORMAL_GA_PACKING: &str = "ga";

/**
 * Sets the stencil buffer value to `0`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ZeroStencilOp")]
pub const ZERO_STENCIL_OP: u32 = 0;

/**
 * Keeps the current value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "KeepStencilOp")]
pub const KEEP_STENCIL_OP: u32 = 7680;

/**
 * Sets the stencil buffer value to the specified reference value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "ReplaceStencilOp")]
pub const REPLACE_STENCIL_OP: u32 = 7681;

/**
 * Increments the current stencil buffer value. Clamps to the maximum representable unsigned value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "IncrementStencilOp")]
pub const INCREMENT_STENCIL_OP: u32 = 7682;

/**
 * Decrements the current stencil buffer value. Clamps to `0`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DecrementStencilOp")]
pub const DECREMENT_STENCIL_OP: u32 = 7683;

/**
 * Increments the current stencil buffer value. Wraps stencil buffer value to zero when incrementing
 * the maximum representable unsigned value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "IncrementWrapStencilOp")]
pub const INCREMENT_WRAP_STENCIL_OP: u32 = 34055;

/**
 * Decrements the current stencil buffer value. Wraps stencil buffer value to the maximum representable
 * unsigned value when decrementing a stencil buffer value of `0`.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DecrementWrapStencilOp")]
pub const DECREMENT_WRAP_STENCIL_OP: u32 = 34056;

/**
 * Inverts the current stencil buffer value bitwise.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InvertStencilOp")]
pub const INVERT_STENCIL_OP: u32 = 5386;

/**
 * Will never return true.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NeverStencilFunc")]
pub const NEVER_STENCIL_FUNC: u32 = 512;

/**
 * Will return true if the stencil reference value is less than the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessStencilFunc")]
pub const LESS_STENCIL_FUNC: u32 = 513;

/**
 * Will return true if the stencil reference value is equal to the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "EqualStencilFunc")]
pub const EQUAL_STENCIL_FUNC: u32 = 514;

/**
 * Will return true if the stencil reference value is less than or equal to the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessEqualStencilFunc")]
pub const LESS_EQUAL_STENCIL_FUNC: u32 = 515;

/**
 * Will return true if the stencil reference value is greater than the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterStencilFunc")]
pub const GREATER_STENCIL_FUNC: u32 = 516;

/**
 * Will return true if the stencil reference value is not equal to the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NotEqualStencilFunc")]
pub const NOT_EQUAL_STENCIL_FUNC: u32 = 517;

/**
 * Will return true if the stencil reference value is greater than or equal to the current stencil value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterEqualStencilFunc")]
pub const GREATER_EQUAL_STENCIL_FUNC: u32 = 518;

/**
 * Will always return true.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AlwaysStencilFunc")]
pub const ALWAYS_STENCIL_FUNC: u32 = 519;

/**
 * Never pass.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NeverCompare")]
pub const NEVER_COMPARE: u32 = 512;

/**
 * Pass if the incoming value is less than the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessCompare")]
pub const LESS_COMPARE: u32 = 513;

/**
 * Pass if the incoming value equals the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "EqualCompare")]
pub const EQUAL_COMPARE: u32 = 514;

/**
 * Pass if the incoming value is less than or equal to the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "LessEqualCompare")]
pub const LESS_EQUAL_COMPARE: u32 = 515;

/**
 * Pass if the incoming value is greater than the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterCompare")]
pub const GREATER_COMPARE: u32 = 516;

/**
 * Pass if the incoming value is not equal to the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "NotEqualCompare")]
pub const NOT_EQUAL_COMPARE: u32 = 517;

/**
 * Pass if the incoming value is greater than or equal to the texture value.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "GreaterEqualCompare")]
pub const GREATER_EQUAL_COMPARE: u32 = 518;

/**
 * Always pass.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "AlwaysCompare")]
pub const ALWAYS_COMPARE: u32 = 519;

/**
 * The contents are intended to be specified once by the application, and used many
 * times as the source for drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StaticDrawUsage")]
pub const STATIC_DRAW_USAGE: u32 = 35044;

/**
 * The contents are intended to be respecified repeatedly by the application, and
 * used many times as the source for drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DynamicDrawUsage")]
pub const DYNAMIC_DRAW_USAGE: u32 = 35048;

/**
 * The contents are intended to be specified once by the application, and used at most
 * a few times as the source for drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StreamDrawUsage")]
pub const STREAM_DRAW_USAGE: u32 = 35040;

/**
 * The contents are intended to be specified once by reading data from the 3D API, and queried
 * many times by the application.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StaticReadUsage")]
pub const STATIC_READ_USAGE: u32 = 35045;

/**
 * The contents are intended to be respecified repeatedly by reading data from the 3D API, and queried
 * many times by the application.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DynamicReadUsage")]
pub const DYNAMIC_READ_USAGE: u32 = 35049;

/**
 * The contents are intended to be specified once by reading data from the 3D API, and queried at most
 * a few times by the application
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StreamReadUsage")]
pub const STREAM_READ_USAGE: u32 = 35041;

/**
 * The contents are intended to be specified once by reading data from the 3D API, and used many times as
 * the source for WebGL drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StaticCopyUsage")]
pub const STATIC_COPY_USAGE: u32 = 35046;

/**
 * The contents are intended to be respecified repeatedly by reading data from the 3D API, and used many times
 * as the source for WebGL drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "DynamicCopyUsage")]
pub const DYNAMIC_COPY_USAGE: u32 = 35050;

/**
 * The contents are intended to be specified once by reading data from the 3D API, and used at most a few times
 * as the source for WebGL drawing and image specification commands.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "StreamCopyUsage")]
pub const STREAM_COPY_USAGE: u32 = 35042;

/**
 * GLSL 1 shader code.
 *
 * @type {string}
 * @constant
 */
pub const GLSL1: &str = "100";

/**
 * GLSL 3 shader code.
 *
 * @type {string}
 * @constant
 */
pub const GLSL3: &str = "300 es";

/**
 * WebGL coordinate system.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "WebGLCoordinateSystem")]
pub const WEB_GL_COORDINATE_SYSTEM: u32 = 2000;

/**
 * WebGPU coordinate system.
 *
 * @type {number}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "WebGPUCoordinateSystem")]
pub const WEB_GPU_COORDINATE_SYSTEM: u32 = 2001;

/**
 * Represents the different timestamp query types.
 *
 * @type {ConstantsTimestampQuery}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "TimestampQuery")]
pub static TIMESTAMP_QUERY: ConstantsTimestampQuery = ConstantsTimestampQuery {
    compute: "compute",
    render: "render",
};

/**
 * Represents mouse buttons and interaction types in context of controls.
 *
 * @type {ConstantsInterpolationSamplingType}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolationSamplingType")]
pub static INTERPOLATION_SAMPLING_TYPE: ConstantsInterpolationSamplingType =
    ConstantsInterpolationSamplingType {
        perspective: "perspective",
        linear: "linear",
        flat: "flat",
    };

/**
 * Represents the different interpolation sampling modes.
 *
 * @type {ConstantsInterpolationSamplingMode}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "InterpolationSamplingMode")]
pub static INTERPOLATION_SAMPLING_MODE: ConstantsInterpolationSamplingMode =
    ConstantsInterpolationSamplingMode {
        normal: "normal",
        centroid: "centroid",
        sample: "sample",
        first: "first",
        either: "either",
    };

/**
 * Compatibility flags for features that may not be supported across all platforms.
 *
 * @type {Object}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "Compatibility")]
pub static COMPATIBILITY: CompatibilityFlags = CompatibilityFlags {
    texture_compare: "depthTextureCompare",
};

/**
 * Represents the refresh types of render objects.
 *
 * @type {ConstantsRenderObjectRefreshType}
 * @constant
 */
#[cfg_attr(rust_js, rust_js::name = "RenderObjectRefreshType")]
pub static RENDER_OBJECT_REFRESH_TYPE: ConstantsRenderObjectRefreshType =
    ConstantsRenderObjectRefreshType {
        none: 0,
        shared: 1,
        full: 2,
    };
