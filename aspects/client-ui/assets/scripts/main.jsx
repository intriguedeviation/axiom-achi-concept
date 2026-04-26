import React from "react";
import { createRoot } from "react-dom/client";
import { App } from "./app";

const rootId = "#root";
const rootContainer = document.querySelector(rootId);

if (rootContainer !== null) {
  const appRoot = createRoot(rootContainer);
  appRoot.render(<App />);
} else {
  console.error("No root container found.");
}
