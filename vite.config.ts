import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import ViteRsw from 'vite-plugin-rsw';

export default defineConfig({
  plugins: [
    solidPlugin(),
    ViteRsw({
      crates: [
        // '@rsw/hey',
        // 'rsw-test',
        // https://github.com/lencx/vite-plugin-rsw/issues/8#issuecomment-820281861
        // outDir: use `path.resolve` or relative path.
        // npm run devでビルドされる出力先を指定
        { name: 'wasm', outDir: './lib' },
      ],
    }),
  ],
  build: {
    target: "esnext",
    polyfillDynamicImport: false,
  },
});
