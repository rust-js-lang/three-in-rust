/**
 * Abstract class for representing pipelines.
 *
 * @private
 * @abstract
 */
#[cfg_attr(rust_js, rust_js::class(es))]
pub struct Pipeline {
    /**
     * The pipeline's cache key.
     *
     * @type {string}
     */
    pub cache_key: String,

    /**
     * How often the pipeline is currently in use.
     *
     * @type {number}
     * @default 0
     */
    pub used_times: f64,
}

#[cfg_attr(rust_js, rust_js::class)]
impl Pipeline {
    /**
     * Constructs a new pipeline.
     *
     * @param {string} cacheKey - The pipeline's cache key.
     */
    pub fn new(cache_key: String) -> Self {
        Self {
            cache_key,
            used_times: 0.0,
        }
    }
}

js::export_default!(Pipeline);
