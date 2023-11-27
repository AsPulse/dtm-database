import { defineConfig } from 'vite';
import KumaUI from '@kuma-ui/vite';
import react from '@vitejs/plugin-react-swc';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), KumaUI()],
});
