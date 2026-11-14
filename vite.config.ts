import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import standalone from 'ajv/dist/standalone/index.js';
import { defineConfig, type Plugin } from 'vite';
import schema from './shared/schemas/action.schema.json';

function actionValidator(): Plugin {
  const id = 'virtual:action-validator';
  return {
    name: 'action-validator',
    resolveId: source => source === id ? '\0' + id : undefined,
    load(source) {
      if (source !== '\0' + id) return;
      const ajv = new Ajv({ allErrors: true, strictRequired: false, code: { source: true, esm: true } });
      addFormats(ajv);
      // Match the Ajv helpers referenced by this schema's generated code.
      const imports = [
        'import ucs2length from "ajv/dist/runtime/ucs2length";',
        'import equal from "ajv/dist/runtime/equal";',
        'import { fullFormats } from "ajv-formats/dist/formats";',
      ].join('\n');
      const helpers: Record<string, string> = {
        'require("ajv/dist/runtime/ucs2length").default': 'ucs2length',
        'require("ajv/dist/runtime/equal").default': 'equal',
        'require("ajv-formats/dist/formats").fullFormats': 'fullFormats',
      };
      const code = standalone(ajv, ajv.compile(schema)).replace(/require\("[^"]+"\)(?:\.[a-zA-Z]+)?/g, match => {
        const helper = helpers[match];
        if (!helper) throw new Error(`Unknown schema validation helper ${match}`);
        return helper;
      });
      return imports + '\n' + code;
    },
  };
}

export default defineConfig({
  plugins: [react(), tailwindcss(), actionValidator()],
  optimizeDeps: { include: ['ajv/dist/runtime/ucs2length', 'ajv/dist/runtime/equal', 'ajv-formats/dist/formats'] },
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: { target: ['es2022', 'safari16'], assetsInlineLimit: 0 },
});
