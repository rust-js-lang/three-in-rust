use crate::renderer_constants::GPU_CHUNK_BYTES;

/**
 * This function is usually called with the length in bytes of an array buffer.
 * It returns an padded value which ensure chunk size alignment according to STD140 layout.
 *
 * @function
 * @param {number} floatLength - The buffer length.
 * @return {number} The padded length.
 */
pub fn get_float_length(float_length: f64) -> f64 {
    // ensure chunk size alignment (STD140 layout)

    float_length + ((GPU_CHUNK_BYTES - (float_length % GPU_CHUNK_BYTES)) % GPU_CHUNK_BYTES)
}

/**
 * Given the count of vectors and their vector length, this function computes
 * a total length in bytes with buffer alignment according to STD140 layout.
 *
 * @function
 * @param {number} count - The number of vectors.
 * @param {number} [vectorLength=4] - The vector length.
 * @return {number} The padded length.
 */
#[cfg_attr(rust_js, rust_js::none_undefined(vector_length))]
pub fn get_vector_length(count: f64, vector_length: Option<f64>) -> f64 {
    let vector_length = vector_length.unwrap_or(4.0);
    let stride_length = get_stride_length(vector_length);

    let float_length = stride_length * count;

    get_float_length(float_length)
}

/**
 * This function is called with a vector length and ensure the computed length
 * matches a predefined stride (in this case `4`).
 *
 * @function
 * @param {number} vectorLength - The vector length.
 * @return {number} The padded length.
 */
pub fn get_stride_length(vector_length: f64) -> f64 {
    let stride_length = 4.0;

    vector_length + ((stride_length - (vector_length % stride_length)) % stride_length)
}
