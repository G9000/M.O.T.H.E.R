// Builds the memory garden page into ../../assets/garden-explorer, which the executable embeds. Every path is
// relative and every file name fixed (src/explorer.rs lists them); nothing is loaded from another host.
import { fileURLToPath } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const names = { entryFileNames: 'assets/[name].js', chunkFileNames: 'assets/[name].js', assetFileNames: 'assets/[name][extname]' };

export default {
  root: here,
  base: './',
  publicDir: 'public',
  resolve: { alias: [{ find: /^patch-block$/, replacement: `${here}src/garden/no-block.js` }] },
  worker: { format: 'es', rolldownOptions: { output: names } },
  build: {
    target: 'es2022',
    outDir: `${here}../../assets/garden-explorer`,
    emptyOutDir: true,
    assetsInlineLimit: 0,
    modulePreload: { polyfill: false },
    chunkSizeWarningLimit: 8000,
    rolldownOptions: { output: names },
    // three's notice, and any other library's, into the page's files
    license: { fileName: 'THIRD-PARTY-NOTICES.md' },
  },
};
