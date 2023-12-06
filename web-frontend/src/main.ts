import App from './App.svelte';

declare const window: any;

const app = new App({
  target: document.body
});

export default app;
