import * as React from "react";

import { invoke } from "@tauri-apps/api/tauri";

export function useFolders(): string[] {
  const [folders, setFolders] = React.useState<string[]>([]);
  const [isPending, startTransition] = React.useTransition();

  React.useEffect(() => {
    startTransition(() => {
      invoke("get_folders")
        .then((store) => setFolders(store as string[]))
        .catch((err) => err); // will figure out later on
    });
  }, []);

  return folders;
}
