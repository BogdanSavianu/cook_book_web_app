import rollup_nre from '@rollup/plugin-node-resolve';
import rollup_tsc from 'rollup-plugin-typescript2';
// import rollup_cjs from '@rollup/plugin-commonjs';
// import { terser } from 'rollup-plugin-terser';

export default [
  {
    input: './src/main.ts',
    output: {
      file: './web-folder/js/app-bundle.js',
      format: 'iife',
      name: 'bundle',
      sourcemap: true
    },
    plugins: [
      // rollup_cjs(),
      rollup_nre(),
      rollup_tsc({
        tsconfig: './tsconfig.json',
        compilerOptions: {
          declaration: false,
          declarationDir: null
        }
      }),
      // terser({ compress: true, format: { comments: false } })
    ]
  }
]
