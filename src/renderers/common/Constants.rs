pub struct AttributeTypes {
    #[cfg_attr(rust_js, rust_js::name = "VERTEX")]
    pub vertex: u32,
    #[cfg_attr(rust_js, rust_js::name = "INDEX")]
    pub index: u32,
    #[cfg_attr(rust_js, rust_js::name = "STORAGE")]
    pub storage: u32,
    #[cfg_attr(rust_js, rust_js::name = "INDIRECT")]
    pub indirect: u32,
}

#[cfg_attr(rust_js, rust_js::name = "AttributeType")]
pub static ATTRIBUTE_TYPE: AttributeTypes = AttributeTypes {
    vertex: 1,
    index: 2,
    storage: 3,
    indirect: 4,
};

// size of a chunk in bytes (STD140 layout)

pub const GPU_CHUNK_BYTES: f64 = 16.0;
