import { defineConfig, searchForWorkspaceRoot } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: [searchForWorkspaceRoot(process.cwd()),
        '../packages/wasm-graph-view/pkg/wasm_graph_view_bg.wasm'],
    },
  },
})
