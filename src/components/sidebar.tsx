import { useQuery } from "@tanstack/react-query";
import { getFolders } from "~/lib/query";

export function Sidebar(props: {
  setCurrentFolder: (folder: string) => void;
}) {
  const folderQuery = useQuery({
    queryKey: ["folders"],
    queryFn: getFolders,
  })

  return (
    <aside className="h-full max-h-screen p-6 border-r border-r-muted max-w-fit">
      <h4 className="text-base text-muted-foreground uppercase font-semibold mb-2 select-none">
        Folders
      </h4>

      <ul className="flex flex-col gap-2">
        {folderQuery.data?.map((folder) => (
          <li
            key={folder}
            onClick={() => props.setCurrentFolder(folder)}
            className="select-none cursor-pointer"
          >
            {folder}
          </li>
        ))}
      </ul>
    </aside>
  );
}
