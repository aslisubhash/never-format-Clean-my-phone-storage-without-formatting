import { useEffect } from "react";
import { useApp } from "../state";
import { formatBytes, type CategoryBreakdown } from "../lib/types";

export function DashboardPage() {
  const { selected, summary, scan, loading, error } = useApp();

  useEffect(() => {
    if (selected && !summary) void scan();
  }, [selected, summary, scan]);

  return (
    <div className="mx-auto max-w-4xl space-y-10">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Dashboard</p>
        <h2 className="mt-2 text-4xl font-semibold tracking-tight text-ink">
          Your phone. Your files. Your PC.
        </h2>
        <p className="mt-3 max-w-xl text-slate">
          No cloud. No format. If we cannot verify an operation is safe, we do nothing.
        </p>
      </header>

      {error && (
        <div className="rounded-md border border-coral/30 bg-coral/10 px-4 py-3 text-sm text-coral">
          {error}
        </div>
      )}

      {!selected && (
        <p className="text-slate">
          Connect a phone via USB with USB debugging enabled. Install Android platform-tools so{" "}
          <code className="text-ink">adb</code> is on your PATH.
          {import.meta.env.DEV && (
            <> Debug builds also show a Mock Galaxy device when no phone is connected.</>
          )}
        </p>
      )}

      {selected?.transport === "mock" && (
        <div className="rounded-md border border-amber/40 bg-amber/10 px-4 py-3 text-sm text-ink">
          Dev mock device — not a real phone. Connect USB debugging for live storage.
        </div>
      )}

      {selected && (
        <>
          <section className="grid gap-6 md:grid-cols-3">
            <Stat
              q="How much storage do I have?"
              a={
                summary
                  ? `${formatBytes(summary.used_bytes)} used · ${formatBytes(summary.free_bytes)} free`
                  : loading
                    ? "Scanning…"
                    : "—"
              }
            />
            <Stat
              q="What's consuming it?"
              a={
                summary?.categories[0]
                  ? summary.categories
                      .slice(0, 3)
                      .map((c: CategoryBreakdown) => `${labelCat(c.category)} ${formatBytes(c.bytes)}`)
                      .join(" · ")
                  : "—"
              }
            />
            <Stat
              q="What can I safely remove?"
              a={
                summary
                  ? `Safe ${formatBytes(summary.safe_to_clean_bytes)} · Backup then ${formatBytes(summary.backup_then_remove_bytes)}`
                  : "—"
              }
            />
          </section>

          <button
            onClick={() => void scan()}
            disabled={loading}
            className="rounded-md bg-pine px-5 py-2.5 text-sm font-medium text-white transition hover:bg-pine-dark disabled:opacity-50"
          >
            {loading ? "Scanning…" : "Analyze storage"}
          </button>
        </>
      )}
    </div>
  );
}

function Stat({ q, a }: { q: string; a: string }) {
  return (
    <div className="border-t border-pine/20 pt-4">
      <div className="text-sm text-slate">{q}</div>
      <div className="mt-2 text-lg font-medium leading-snug text-ink">{a}</div>
    </div>
  );
}

function labelCat(c: string) {
  return c.replace(/_/g, " ");
}
