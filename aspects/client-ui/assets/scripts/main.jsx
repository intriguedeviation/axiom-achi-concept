import React from "react";
import { createRoot } from "react-dom/client";
import initAchi, { initialize } from "achi";
import achiWasmUrl from "achi/achi_bg.wasm?url";

const rootId = "#root";
const rootContainer = document.querySelector(rootId);

const start = async () => {
  await initAchi(achiWasmUrl);
  initialize();
  const { App } = await import("./app");

  const appRoot = createRoot(rootContainer);
  appRoot.render(<App />);
};

if (rootContainer !== null) {
  start().catch((error) => {
    console.error("Unable to initialize Achi.", error);
  });
} else {
  console.error("No root container found.");
}
