import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';
import Players from './Players.svelte';

const routes: Record<string, any> = {
  '/': App,
  '/players': Players,
};

const component = routes[window.location.pathname] ?? App;

const app = mount(component, {
  target: document.getElementById('app')!,
});

export default app;
