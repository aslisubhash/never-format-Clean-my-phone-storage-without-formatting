import { useEffect, useState } from "react";
import { api } from "../lib/api";
import { useApp } from "../state";
import type { Operation, ScannedFile } from "../lib/types";
import { formatBytes } from "../lib/types";

export function BackupPage() {
  const { selected, scan } = useApp();
  const [files, setFiles] = useState<ScannedFile[]>([]);
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());
  const [dest, setDest] = useState("");
  const [op, setOp] = useState<Operation>();
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    void (async () => {
      if (selected) await scan();
      setFiles(await api.listFiles());
      setDest(await api.getBackupDest());
    })();
  }, [selected]);

  const toggle = (path: string) => {
    setSelectedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  const run = async (dryRun: boolean) => {
    if (!selected) return;
    setBusy(true);
    try {
      const result = await api.runBackup(selected.id, [...selectedPaths], dryRun);
      setOp(result);
    } finally {
      setBusy(false);
    }
  };

  const backupable = files.filter(
    (f) => f.safety === "safe_after_backup" || f.safety === "review_required",
  );

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Backup</p>
        <h2 className="mt-2 text-3xl font-semibold">Verified copy to PC</h2>
        <p className="mt-2 text-slate">
          Files are hashed after copy. Deletion is never implied by backup.
        </p>
      </header>

      <label className="block text-sm">
        <span className="text-slate">Backup destination root</span>
        <input
          className="mt-1 w-full rounded-md border border-pine/20 bg-white/70 px-3 py-2"
          value={dest}
          onChange={(e) => setDest(e.target.value)}
          onBlur={() => void api.setBackupDest(dest)}
        />
      </label>

      <ul className="divide-y divide-pine/10 border-y border-pine/10">
        {backupable.map((f) => (
          <li key={f.path} className="flex items-center gap-3 py-3">
            <input
              type="checkbox"
              checked={selectedPaths.has(f.path)}
              onChange={() => toggle(f.path)}
            />
            <div className="min-w-0 flex-1">
              <div className="font-medium">{f.name}</div>
              <div className="truncate text-xs text-slate">{f.path}</div>
            </div>
            <span className="tabular-nums text-sm">{formatBytes(f.size_bytes)}</span>
          </li>
        ))}
      </ul>

      <div className="flex gap-3">
        <button
          disabled={!selectedPaths.size || busy}
          onClick={() => void run(true)}
          className="rounded-md border border-pine/30 px-4 py-2 text-sm disabled:opacity-40"
        >
          Dry-run plan
        </button>
        <button
          disabled={!selectedPaths.size || busy}
          onClick={() => void run(false)}
          className="rounded-md bg-pine px-4 py-2 text-sm text-white disabled:opacity-40"
        >
          Backup & verify
        </button>
      </div>

      {op && (
        <pre className="overflow-auto rounded-md bg-ink p-4 text-xs text-paper">
          {JSON.stringify(op, null, 2)}
        </pre>
      )}
    </div>
  );
}
