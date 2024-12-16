import { resolve } from 'path';
import dts from 'vite-plugin-dts';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vite';

export default defineConfig({
	build: {
		lib: {
			entry: resolve(__dirname, 'src/main.ts'),
			fileName: `harper.js`,
			name: 'harper',
			formats: ['es']
		}
	},
	base: './',
	plugins: [wasm(), topLevelAwait(), dts({ rollupTypes: true, tsconfigPath: './tsconfig.json' })],
	worker: {
		plugins: [wasm(), topLevelAwait()],
		format: 'es'
	},
	test: {
		browser: {
			provider: 'playwright',
			enabled: true,
			name: 'chromium'
		}
	}
});
