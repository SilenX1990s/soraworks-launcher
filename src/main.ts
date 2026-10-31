import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { theme } from "./lib/stores/theme";
import { get } from "svelte/store";
import { applyTheme } from "./lib/stores/theme";

// Apply the saved theme before first paint to avoid a flash.
applyTheme(get(theme));

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
