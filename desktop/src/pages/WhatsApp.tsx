import { useEffect, useState } from "react";
import { api } from "../lib/api";
import { useApp } from "../state";
import type { WhatsAppBackupInfo } from "../lib/types";
import { formatBytes, safetyColor, safetyLabel } from "../lib/types";

export function WhatsAppPage() {
  const { selected } = useApp();
  const [items, setItems] = useState<WhatsAppBackupInfo[]>([]);

  useEffect(() => {
    if (!selected) return;
    void api.listWhatsapp(selected.id).then(setItems);
  }, [selected]);

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">WhatsApp</p>
        <h2 className="mt-2 text-3xl font-semibold">Backup management</h2>
        <p className="mt-2 text-slate">
          The latest database is always protected. Older backups may be removed only after
          verified PC backup.
        </p>
      </header>

      <ul className="divide-y divide-pine/10 border-y border-pine/10">
        {items.map((b) => (
          <li key={b.path} className="flex items-center justify-between gap-4 py-4">
            <div className="min-w-0">
              <div className="font-medium truncate">{b.path}</div>
              <div className="text-xs text-slate">
                {b.is_latest ? "Latest — protected" : "Older backup"}
              </div>
            </div>
            <div className="flex items-center gap-3">
              <span className={`rounded px-2 py-0.5 text-xs ${safetyColor(b.safety)}`}>
                {safetyLabel(b.safety)}
              </span>
              <span className="tabular-nums text-sm">{formatBytes(b.size_bytes)}</span>
            </div>
          </li>
        ))}
        {!items.length && <li className="py-4 text-slate">No WhatsApp backups discovered.</li>}
      </ul>
    </div>
  );
}
