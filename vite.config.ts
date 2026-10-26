import {defineConfig} from 'vite';
import react from '@vitejs/plugin-react';
export default defineConfig({plugins:[react()],clearScreen:false,server:{strictPort:true,host:'localhost',port:1420},envPrefix:['VITE_','TAURI_ENV_*'],build:{target:'chrome105'}});
