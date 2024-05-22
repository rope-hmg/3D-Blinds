import "main.css";

import App from "App.svelte";
import init from "app";

export const initialised = new Promise((resolve, reject) => init().then(resolve).catch(reject));

new App({
    target: document.getElementById("app")!,
});
