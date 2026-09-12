import { useState } from "react";
import { api } from "../lib/api";
import { useApp } from "../state";
import type { Operation } from "../lib/types";
import { formatBytes } from "../lib/types";

export function CleanupPage() {
  const { selected, scan } = useApp();
  const [op, setOp] = useState<Operation>();
  const [busy, setBusy] = useState(false);

  const dryRun = async () => {
    if (!selected) return;
    setBusy(true);
    try {
      await scan();
      setOp(await api.planCleanup(selected.id, true));
    } finally {
      setBusy(false);
    }
  };

  const execute = async () => {
    if (!selected) return;
    const ok = window.confirm(
      "Delete only Safe-to-clean items that passed policy checks? This cannot be undone on the device.",
    );
    if (!ok) return;
    setBusy(true);
    try {
      setOp(await api.runCleanup(selected.id, true));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Cleanup</p>
        <h2 className="mt-2 text-3xl font-semibold">Safe cleanup</h2>
        <p className="mt-2 text-slate">
          Protected and Unknown items are never deleted. Prefer dry-run first.
        </p>
      </header>

      <div className="flex gap-3">
        <button
          disabled={busy || !selected}
          onClick={() => void dryRun()}
          className="rounded-md border border-pine/30 px-4 py-2 text-sm"
        >
          Dry-run plan
        </button>
        <button
          disabled={busy || !selected}
          onClick={() => void execute()}
          className="rounded-md bg-pine px-4 py-2 text-sm text-white"
        >
          Execute safe cleanup
        </button>
      </div>

      {op && (
        <div className="space-y-3">
          <p className="text-sm">
            {op.message} ({op.items.length} items)
          </p>
          <ul className="divide-y divide-pine/10 border-y border-pine/10 text-sm">
            {op.items.map((i) => (
              <li key={i.source_path} className="flex justify-between py-2">
                <span className="truncate">{i.source_path}</span>
                <span className="tabular-nums text-slate">{formatBytes(i.size_bytes)}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
