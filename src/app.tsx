import * as React from "react";
import { Item } from "~/components/item";
import { Sidebar } from "~/components/sidebar";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";

import { useQuery } from "@tanstack/react-query";
import { toast } from "react-hot-toast";
import { getFolders, getPasswords } from "~/lib/query";

export function App() {
  const folderQuery = useQuery({
    queryKey: ["folders"],
    queryFn: getFolders,
  })

  const [currentFolder, setCurrentFolder] = React.useState<string>();

  const passwordsQuery = useQuery({
    queryKey: ["passwords", currentFolder],
    queryFn: () => getPasswords(currentFolder || "internet"),
  })

  React.useEffect(() => {
    setCurrentFolder(folderQuery.data?.at(0) || "")
  }, []);

  if (folderQuery.isError) {
    toast.error("Unable to get your password vault");
  }

  if (passwordsQuery.isError) {
    toast.error("Unable to get your passwords")
  }

  return (
    <div className="flex h-full">
      <Sidebar setCurrentFolder={setCurrentFolder} />
      <main className="flex-1 flex flex-col gap-2 p-6">
        <h1 className="text-lg font-semibold">
          {currentFolder}
        </h1>
        <div className="flex gap-4 mb-2 mt-1">
          <Input placeholder="Search..." className="max-w-sm" />
          <Button variant="secondary">
            New Password
          </Button>
        </div>
        <ul className="flex flex-col gap-4">
          {passwordsQuery.isLoading
            ? "Loading... "
            : passwordsQuery.data?.map((password) => (
                <Item key={password.filename} data={password} />
              ))}
        </ul>
      </main>
    </div>
  );
}
