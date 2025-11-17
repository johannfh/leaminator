import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
    plugins: [svelte()],
    // Vite development server configuration
    server: {
        port: 3000,
        proxy: {
            '^/ws/.*': {
                target: 'http://localhost:8000',
                changeOrigin: true,
                ws: true,
                secure: false,
            },
            '^/api/.*': {
                target: 'http://localhost:8000',
                changeOrigin: true,
                secure: false,
            },
        },
    },
})
