use std::cell::Cell;

thread_local! {
    #[cfg_attr(rust_js, rust_js::name = "_id")]
    static ID: Cell<f64> = const { Cell::new(0.0) };
}

#[derive(Clone, Copy)]
pub enum ShaderStage {
    #[cfg_attr(rust_js, rust_js::name = "vertex")]
    Vertex,
    #[cfg_attr(rust_js, rust_js::name = "fragment")]
    Fragment,
    #[cfg_attr(rust_js, rust_js::name = "compute")]
    Compute,
}

/**
 * Class for representing programmable stages which are vertex,
 * fragment or compute shaders. Unlike fixed-function states (like blending),
 * they represent the programmable part of a pipeline.
 *
 * @private
 */
#[cfg_attr(rust_js, rust_js::class(es))]
pub struct ProgrammableStage {
    /**
     * The id of the programmable stage.
     *
     * @type {number}
     */
    pub id: f64,
    /**
     * The shader code.
     *
     * @type {string}
     */
    pub code: String,
    /**
     * The type of stage.
     *
     * @type {string}
     */
    pub stage: ShaderStage,
    /**
     * The name of the stage.
     * This is used for debugging purposes.
     *
     * @type {string}
     */
    pub name: String,
    /**
     * How often the programmable stage is currently in use.
     *
     * @type {number}
     * @default 0
     */
    pub used_times: f64,
}

#[cfg_attr(rust_js, rust_js::class)]
impl ProgrammableStage {
    /**
     * Constructs a new programmable stage.
     *
     * @param {string} code - The shader code.
     * @param {('vertex'|'fragment'|'compute')} stage - The type of stage.
     * @param {string} name - The name of the shader.
     */
    pub fn new(code: String, stage: ShaderStage, name: String) -> Self {
        Self {
            id: ID.replace(ID.get() + 1.0),
            code,
            stage,
            name,
            used_times: 0.0,
        }
    }
}

js::export_default!(ProgrammableStage);
