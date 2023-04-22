import * as React from "react";
import { PlusIcon } from "@heroicons/react/20/solid";
import { Button } from "~/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";

import { invoke } from "@tauri-apps/api/tauri";
import { toast } from "react-hot-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export function Nav() {
  const queryClient = useQueryClient();
  const [folderName, setFolderName] = React.useState<string>("");
  const mutation = useMutation({
    mutationFn: () => {
      return invoke("add_folder", { name: folderName })
        .then(() => {
          toast.success("Folder created");
          setFolderName("");
        })
        .catch(() => toast.error("Unable to create folder"));
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["folders"]})
    }
  })

  return (
    <nav className="flex justify-between py-4 px-6 border-b border-b-muted items-center">
      <h3 className="text-lg font-bold">pass.</h3>
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="secondary">
            <PlusIcon className="w-6 h-6 mr-1" /> New Folder
          </Button>
        </DialogTrigger>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Add New Folder</DialogTitle>
            <DialogDescription>
              Add a new folder to your password store.
            </DialogDescription>
          </DialogHeader>
          <form
            onSubmit={() => mutation.mutate()}
          >
            <div className="grid gap-4 py-4">
              <div className="flex ml-1 items-center gap-4">
                <Label htmlFor="name" className="text-right">
                  Name
                </Label>
                <Input
                  id="name"
                  value={folderName}
                  onChange={(evt) => setFolderName(evt.target.value)}
                  className="col-span-3"
                />
              </div>
            </div>
            <DialogFooter>
              <Button type="submit" disabled={mutation.isLoading}>Create Folder</Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </nav>
  );
}
