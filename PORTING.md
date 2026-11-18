# Three.js in Rust

This fork ports Three.js to Rust with [rust-js](https://github.com/rust-js-lang/rust-js). Each checkpoint keeps the Three.js JavaScript entry points usable while more modules move to Rust.

The starting revision is `acf77e4c7ea413a0a9638baacef3d78e254b2cac` of Three.js. The original test cases and rendering references are the compatibility checks for the port.

## Verification

Initialize the compiler submodule with `git submodule update --init rust-js`, then install dependencies with `bun install --frozen-lockfile`. Puppeteer needs Chrome. To use a system installation, set `PUPPETEER_EXECUTABLE_PATH` to its executable.

- `bun run test` runs core lint and the original core and addon unit suites. The starting revision passed 1,987 core tests and 147 addon tests. The upstream `Mesh.raycast` TODO remains.
- `bun run test-e2e-cov` verifies that examples are covered by screenshots and descriptions.
- `bun run build` followed by `bun run test-e2e` runs the rendering comparisons against fresh bundles. Upstream runs the five screenshot shards on Linux with Vulkan drivers and Xvfb.
- `bun run test-treeshake` runs the upstream tree-shaking build checks.

On macOS with Chrome 158, the 562-example screenshot suite had 109 reference mismatches on both the port and the unmodified upstream revision. The port also reported a one-off glTF loading error. The same glTF example passed on rerun with a 0.0% pixel difference. The five rebuilt ESM bundles matched upstream after removing formatting and comments.

In Apple Container, Ubuntu’s signed package sources supplied the Vulkan dependencies and Puppeteer installed its pinned Chrome 156.0.8078.47. The Bun-based setup omits the optional NodeSource feed. Package installation completed without certificate warnings. In a four-example comparison, stock Three.js and the port each matched three reference screenshots exactly. Both measured a 4.2% difference for `webgpu_hdr`.

## Rebuild the Rust modules

The `rust-js` submodule pins the compiler and its bindings. The current pin includes the four compiler fixes upstream. If a future port needs a local compiler fix, the build applies patches in `patches/rust-js/` in filename order.

```sh
git submodule update --init rust-js
bun install --frozen-lockfile
bun install --cwd rust-js --frozen-lockfile --ignore-scripts
bun run build:rust
bun run build:rust --check
```

The compiler uses the Rust toolchain pinned in the submodule. The generated JavaScript and declarations are committed beside their Rust source. Source maps are generated locally. Three.js imports the JavaScript from its original path.

## Ported modules

- `src/renderers/common/Constants.js` defines the renderer's attribute kinds and STD140 chunk size.
- `src/constants.js` defines the 211 public Three.js constants. The export values and object property descriptors match upstream, and all 212 JSDoc blocks are preserved. Three object literals wrap differently, and the six standalone typedefs appear together at the top of the module.
- `src/renderers/webgl-fallback/utils/WebGLConstants.js` maps 11 WebGL extensions to their feature names. The generated keys retain the original names, values, order, and property descriptors.
- `src/renderers/common/Pipeline.js` is the base class for renderer pipelines. Its constructor and the four upstream JSDoc blocks are preserved. The generated class declares the default export inline.
- `src/renderers/common/ProgrammableStage.js` stores shader code and stage metadata. Its private counter and constructor match upstream, including the counter sequence observed by JavaScript subclasses. The Rust enum emits the same three stage strings.
- `src/renderers/common/BufferUtils.js` computes STD140 alignment. Its three helpers preserve the original documentation, JavaScript parameter defaults, and results.
- `src/renderers/common/Binding.js` is the base class for resource bindings. Its constructor default, visibility bitmask, and clone behavior match upstream, including when a subclass supplies the constructor.
- `src/extras/core/Interpolations.js` evaluates Catmull–Rom splines and quadratic and cubic Bézier curves. The three exported functions, their four JSDoc blocks, and 1,536 sampled results match the original and native Rust.
