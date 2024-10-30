import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react(), tsconfigPaths()],
    server: {
        proxy: {
            '/api': {
                target: "http://localhost:8000",
                changeOrigin: true,
                secure: false,
                ws: true
            }
        }
    },
    build: {
        rollupOptions: {
            output: {
                manualChunks: (id) => {
                    if (id.indexOf('@nextui-org') !== -1) {
                        return 'nextui';
                    }
                    if (id.indexOf('@react') !== -1) {
                        return 'react';
                    }
                    if (id.indexOf('node_modules') !== -1) {
                        return 'vendor';
                    }
                }
            }
        }
    }
})
