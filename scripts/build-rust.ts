import { existsSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

const root = resolve(import.meta.dir, '..');
const compiler = join(root, 'rust-js');
const cli = join(compiler, 'tooling', 'cli.js');
const patches = join(root, 'patches', 'rust-js');

if (!existsSync(cli)) {
	throw new Error('Initialize the compiler with git submodule update --init rust-js.');
}

for (const name of readdirSync(patches).filter(name => name.endsWith('.patch')).sort()) {
	const path = join(patches, name);
	const command = ['git', '-C', compiler, 'apply'];
	const applied = Bun.spawnSync([...command, '--reverse', '--check', path], { stderr: 'ignore' });
	if (applied.success) continue;

	const result = Bun.spawnSync([...command, '--whitespace=error', path], { stderr: 'inherit' });
	if (!result.success) throw new Error(`Could not apply ${name} to the rust-js submodule.`);
}

const build = Bun.spawn([process.execPath, cli, 'build', ...process.argv.slice(2)], {
	cwd: root,
	stdio: ['inherit', 'inherit', 'inherit'],
});
process.exitCode = await build.exited;
