use js::Unknown;

/**
 * A binding represents the connection between a resource (like a texture, sampler
 * or uniform buffer) and the resource definition in a shader stage.
 *
 * This module is an abstract base class for all concrete bindings types.
 *
 * @abstract
 * @private
 */
#[cfg_attr(rust_js, rust_js::class(es))]
pub struct Binding {
    /**
     * The binding's name.
     *
     * @type {string}
     */
    pub name: &'static str,

    /**
     * A bitmask that defines in what shader stages the
     * binding's resource is accessible.
     *
     * @type {number}
     */
    pub visibility: i32,
}

#[cfg_attr(rust_js, rust_js::class)]
impl Binding {
    /**
     * Constructs a new binding.
     *
     * @param {string} [name=''] - The binding's name.
     */
    #[cfg_attr(rust_js, rust_js::none_undefined(name))]
    pub fn new(name: Option<&'static str>) -> Self {
        let name = name.unwrap_or("");
        Self {
            name,
            visibility: 0,
        }
    }

    /**
     * Makes sure binding's resource is visible for the given shader stage.
     *
     * @param {number} visibility - The shader stage.
     */
    pub fn set_visibility(&mut self, visibility: i32) {
        self.visibility |= visibility;
    }

    /**
     * Clones the binding.
     *
     * @return {Binding} The cloned binding.
     */
    #[cfg_attr(rust_js, rust_js::name = "clone")]
    pub fn clone_binding(&self) -> &'static Unknown {
        js::object::assign(js::new(&js::get(self, "constructor"), ()), (self,))
    }
}

js::export_default!(Binding);
