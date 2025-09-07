import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	optimizeDeps: { include: ['@skeletonlabs/skeleton-svelte', '@skeletonlabs/skeleton', 'lucide-svelte'] },
	// Tauri expects a fixed port, fail if that port is not available
	server: {
		port: 5173,
		strictPort: true,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ['**/src-tauri/**']
		}
	}
});
