import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    // Enable source maps for debugging
    sourcemap: true,
    // Optimize chunking for large vendor libraries
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (id.includes('react') || id.includes('react-dom')) {
              return 'react-vendor'
            }
            if (id.includes('motion') || id.includes('motion-dom') || id.includes('motion-utils')) {
              return 'animation-vendor'
            }
            if (id.includes('lenis')) {
              return 'scroll-vendor'
            }
          }
        },
      },
    },
    // Enable CSS code splitting
    cssCodeSplit: true,
    // Use esbuild minifier (built-in, no extra deps needed)
  },
})
