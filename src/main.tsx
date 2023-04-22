import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider} from "@tanstack/router";

import { router } from "~/lib/router";

import "./styles/globals.css";
import "./styles/fonts.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
