# Three.js in Rust

This fork ports Three.js to Rust with [rust-js](https://github.com/rust-js-lang/rust-js). Each checkpoint keeps the Three.js JavaScript entry points usable while more modules move to Rust.

The starting revision is `acf77e4c7ea413a0a9638baacef3d78e254b2cac` of Three.js. The original test cases and rendering references are the compatibility checks for the port.

## Verification

Install dependencies with `bun install --frozen-lockfile`. Puppeteer needs Chrome. To use a system installation, set `PUPPETEER_EXECUTABLE_PATH` to its executable.

- `bun run test` runs core lint and the original core and addon unit suites. The starting revision passed 1,987 core tests and 147 addon tests. The upstream `Mesh.raycast` TODO remains.
- `bun run test-e2e-cov` verifies that examples are covered by screenshots and descriptions.
- `bun run build` followed by `bun run test-e2e` runs the rendering comparisons against fresh bundles. Upstream runs the five screenshot shards on Linux with Vulkan drivers and Xvfb.
- `bun run test-treeshake` runs the upstream tree-shaking build checks.
