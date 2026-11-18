import MagicString from 'magic-string';

function glsl() {

	return {

		transform( code, id ) {

			if ( /\.glsl.js$/.test( id ) === false ) return;

			const source = new MagicString( code );

			for ( const statement of this.parse( code ).body ) {

				const declaration = statement.declaration;
				const expressions = statement.type === 'ExportDefaultDeclaration' ? [ declaration ] :
					declaration?.type === 'VariableDeclaration' ? declaration.declarations.map( entry => entry.init ) : [];

				for ( const expression of expressions ) {

					if ( expression?.type !== 'TemplateLiteral' || expression.expressions.length !== 0 ) continue;

					const prefix = code.slice( statement.start, expression.start );
					const marker = /\/\* glsl \*\/\s*$/.exec( prefix );
					if ( ! marker ) continue;

					// Use the shader string as JavaScript reads it, including escaped tabs.
					const shader = expression.quasis[ 0 ].value.cooked
						.trim()
						.replace( /\r/g, '' )
						.replace( /[ \t]*\/\/.*\n/g, '' ) // remove //
						.replace( /[ \t]*\/\*[\s\S]*?\*\//g, '' ) // remove /* */
						.replace( /\n{2,}/g, '\n' ); // # \n+ to \n

					source.overwrite( statement.start + marker.index, expression.end, JSON.stringify( shader ) );

				}

			}

			return {
				code: source.toString(),
				map: source.generateMap()
			};

		}

	};

}

function header() {

	return {

		renderChunk( code ) {

			code = new MagicString( code );

			code.prepend( `/**
 * @license
 * Copyright 2010-2026 Three.js Authors
 * SPDX-License-Identifier: MIT
 */\n` );

			return {
				code: code.toString(),
				map: code.generateMap()
			};

		}

	};

}

/**
 * @type {Array<import('rollup').RollupOptions>}
 */
const builds = [
	{
		input: {
			'three.core.js': 'src/Three.Core.js',
			'three.webgpu.nodes.js': 'src/Three.WebGPU.Nodes.js',
		},
		plugins: [
			glsl(),
			header()
		],
		preserveEntrySignatures: 'allow-extension',
		output: [
			{
				format: 'esm',
				dir: 'build',
				minifyInternalExports: false,
				entryFileNames: '[name]',
			}
		]
	},
	{
		input: {
			'three.core.js': 'src/Three.Core.js',
			'three.module.js': 'src/Three.js',
			'three.webgpu.js': 'src/Three.WebGPU.js',
			'three.tsl.js': 'src/Three.TSL.js',
		},
		plugins: [
			glsl(),
			header()
		],
		preserveEntrySignatures: 'allow-extension',
		output: [
			{
				format: 'esm',
				dir: 'build',
				minifyInternalExports: false,
				entryFileNames: '[name]',
			}
		]
	}
];

export default builds;
