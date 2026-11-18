# Three.js in Rust

This fork ports Three.js to Rust with [rust-js](https://github.com/rust-js-lang/rust-js). Each checkpoint keeps the Three.js JavaScript entry points usable while more modules move to Rust.

The starting revision is `acf77e4c7ea413a0a9638baacef3d78e254b2cac` of Three.js. The original test cases and rendering references are the compatibility checks for the port.

## Verification

Install dependencies with `bun install --frozen-lockfile`. Puppeteer needs Chrome. To use a system installation, set `PUPPETEER_EXECUTABLE_PATH` to its executable.

- `bun run test` runs core lint and the original core and addon unit suites. The starting revision passed 1,987 core tests and 147 addon tests. The upstream `Mesh.raycast` TODO remains.
- `bun run test-e2e-cov` verifies that examples are covered by screenshots and descriptions.
- `bun run build` followed by `bun run test-e2e` runs the rendering comparisons against fresh bundles. Upstream runs the five screenshot shards on Linux with Vulkan drivers and Xvfb.
- `bun run test-treeshake` runs the upstream tree-shaking build checks.

## Rebuild the Rust modules

The compiler revision used for this checkpoint is `ae378c0fc55459c2fc4b4796502a74bec5cd4f5c`. Build it from a local rust-js checkout with its pinned Rust toolchain. Set `RUST_JS_CHECKOUT` to that checkout's absolute path and run these commands from the port:

```sh
bun install --frozen-lockfile
mkdir -p node_modules/@rust-js
ln -s "$RUST_JS_CHECKOUT/std" node_modules/@rust-js/std
bun run build:rust
bun run build:rust --check
```

The link gives the compiler the matching standard library from its checkout. The generated JavaScript and declarations are committed beside the Rust source. Source maps are generated locally. Three.js imports the JavaScript from its original path.

## Ported modules

- `src/renderers/common/Constants.js` defines the renderer's attribute kinds and STD140 chunk size.
- `src/constants.js` defines the 211 public Three.js constants. The export values and object property descriptors match upstream, and all 212 JSDoc blocks are preserved. Three object literals wrap differently, and the six standalone typedefs appear together at the top of the module.
- `src/renderers/webgl-fallback/utils/WebGLConstants.js` maps 11 WebGL extensions to their feature names. The generated keys retain the original names, values, order, and property descriptors.
