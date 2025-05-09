import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import css from 'rollup-plugin-css-only';
import replace from '@rollup/plugin-replace';

const production = !process.env.ROLLUP_WATCH;

function serve() {
  let server;

  function toExit() {
    if (server) server.kill(0);
  }

  return {
    writeBundle() {
      if (server) return;
      server = require('child_process').spawn(
        'npm',
        ['run', 'start', '--', '--dev', '--single'],
        {
          stdio: ['ignore', 'inherit', 'inherit'],
          shell: true,
        },
      );

      process.on('SIGTERM', toExit);
      process.on('exit', toExit);
    },
  };
}

export default {
  input: 'src/main.ts',
  output: {
    sourcemap: true,
    format: 'umd',
    name: 'app',
    file: 'public/build/bundle.js',
  },
  plugins: [
    replace({
      'process.env.BASE_URL': production
        ? JSON.stringify('PLACEHOLDER')
        : JSON.stringify('http://localhost:3000/'),
    }),
    svelte({
      preprocess: sveltePreprocess({ sourceMap: !production }),
      compilerOptions: {
        // enable run-time checks when not in production
        dev: !production,
      },
    }),
    // we'll extract any component CSS out into a separate file - better for performance
    css({ output: 'bundle.css' }),

    // If you have external dependencies installed from npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration - consult the documentation for details:
    // https://github.com/rollup/plugins/tree/master/packages/commonjs
    nodeResolve({
      browser: true,
      dedupe: ['svelte'],
    }),
    commonjs(),
    typescript({
      sourceMap: !production,
      inlineSources: !production,
    }),

    // In dev mode, call `npm run start` once the bundle has been generated
    !production && serve(),

    // Watch the `public` directory and refresh the browser on changes when not in production
    !production && livereload('public'),

    // For production builds minify, remove comments and logs
    production &&
      terser({
        format: {
          comments: false,
        },
        compress: {
          drop_console: true,
        },
      }),
  ],
  watch: {
    clearScreen: false,
  },
};
