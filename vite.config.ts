import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  // Tauri: não usar polling desnecessário
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // Ignora src-tauri para não recompiliar Rust toda vez
      ignored: ['**/src-tauri/**']
    }
  },
  // Evitar erros de CORS em Tauri WebView
  clearScreen: false
});
