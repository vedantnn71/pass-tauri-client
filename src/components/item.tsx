import * as React from "react";
import { type Item } from "~/types";
import {
  LockClosedIcon,
  ChevronDownIcon,
  ClipboardDocumentIcon,
  CheckIcon,
} from "@heroicons/react/20/solid";

import { writeText } from "@tauri-apps/api/clipboard";
import { cn } from "~/lib/utils";

export function Item(props: { data: Item }) {
  const [open, setOpen] = React.useState(false);
  const [copied, setCopied] = React.useState(false);
  const subtitle = props.data.url || props.data.email || props.data.phone;

  function copyPassword() {
    writeText(props.data.password);
    setCopied(true);
    setTimeout(() => setCopied(false), 1000);
  }

  return (
    <li key={props.data.username || props.data.email || props.data.url}>
      <div
        className={cn(
          "p-4 border border-muted rounded-xl flex gap-4",
          open ? "rounded-b-none" : "rounded-b-xl flex-row"
        )}
      >
        <div>
          <div className="flex items-center gap-2 p-3 bg-muted rounded-lg">
            <LockClosedIcon className="w-5 h-5" />
          </div>
        </div>
        <div className="flex items-center gap-2 text-lg">
          <span className="font-medium">{props.data.filename}</span>
          <span className="text-muted-foreground">
            {subtitle ? `− ${subtitle}` : null}
          </span>
        </div>
        <div className="ml-auto flex items-center gap-2 text-muted-foreground">
          <button onClick={() => copyPassword()}>
            {copied ? (
              <CheckIcon className="w-5 h-5" />
            ) : (
              <ClipboardDocumentIcon className="w-5 h-5" />
            )}
          </button>

          <span>
            {open ? (
              <ChevronDownIcon
                className="w-5 h-5 transform rotate-180"
                onClick={() => setOpen(false)}
              />
            ) : (
              <ChevronDownIcon
                className="w-5 h-5"
                onClick={() => setOpen(true)}
              />
            )}
          </span>
        </div>
      </div>
      {open ? (
        <div className="p-4 border border-muted rounded-b-xl flex flex-col gap-2">
          <ItemDetailSection title="username" value={props.data.username} />
          <ItemDetailSection
            title="password"
            value={props.data.password}
            hide={true}
          />
          <ItemDetailSection title="url" value={props.data.url} />
          <ItemDetailSection title="email" value={props.data.email} />
          <ItemDetailSection title="phone" value={props.data.phone} />
          <ItemDetailSection title="notes" value={props.data.notes} />
        </div>
      ) : null}
    </li>
  );
}

function ItemDetailSection(props: {
  title: string;
  value?: string;
  hide?: boolean;
}) {
  if (!props.value) {
    return null;
  }

  return (
    <div className="flex gap-2">
      <span className="text-muted-foreground">{props.title} –</span>
      <button className="font-medium" onClick={() => writeText(props.value!)}>
        {props.hide ? "•".repeat(10) : props.value}
      </button>
    </div>
  );
}
