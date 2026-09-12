import { useEffect, useState } from "react";
import { api } from "../lib/api";
import { useApp } from "../state";
import type { Operation, ScannedFile } from "../lib/types";
import { formatBytes } from "../lib/types";

export function MovePage() {
  const { selected, scan } = useApp();
  const [files, setFiles] = useState<ScannedFile[]>([]);
  const [paths, setPaths] = useState<string[]>([]);
  const [op, setOp] = useState<Operation>();
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    void (async () => {
      if (selected) await scan();
      const list = await api.listFiles();
      setFiles(list.filter((f) => f.safety === "safe_after_backup"));
    })();
  }, [selected]);

  const start = async (confirmed: boolean) => {
    if (!selected || !paths.length) return;
    setBusy(true);
    try {
      setOp(await api.runMove(selected.id, paths, confirmed));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Move</p>
        <h2 className="mt-2 text-3xl font-semibold">Verified move</h2>
        <p className="mt-2 text-slate">
          Copy → Verify (SHA-256) → Confirm → Delete. Never a single-step move.
        </p>
      </header>

      <select
        multiple
        className="h-48 w-full rounded-md border border-pine/20 bg-white/70 p-2 text-sm"
        value={paths}
        onChange={(e) =>
          setPaths([...e.target.selectedOptions].map((o) => o.value))
        }
      >
        {files.map((f) => (
          <option key={f.path} value={f.path}>
            {f.name} ({formatBytes(f.size_bytes)})
          </option>
        ))}
      </select>

      <div className="flex gap-3">
        <button
          disabled={busy || !paths.length}
          onClick={() => void start(false)}
          className="rounded-md border border-pine/30 px-4 py-2 text-sm"
        >
          Copy & verify
        </button>
        <button
          disabled={busy || !paths.length || op?.state !== "awaiting_confirmation"}
          onClick={() => void start(true)}
          className="rounded-md bg-coral px-4 py-2 text-sm text-white disabled:opacity-40"
        >
          Confirm delete from phone
        </button>
      </div>

      {op && (
        <div className="rounded-md border border-amber/30 bg-amber/10 px-4 py-3 text-sm">
          State: <strong>{op.state}</strong> — {op.message}
        </div>
      )}
    </div>
  );
}
