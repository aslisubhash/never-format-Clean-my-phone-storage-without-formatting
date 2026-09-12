import { useApp } from "../state";
import { formatBytes, safetyColor, safetyLabel, type CategoryBreakdown } from "../lib/types";

export function StoragePage() {
  const { summary, scan, loading } = useApp();

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header className="flex items-end justify-between gap-4">
        <div>
          <p className="text-sm uppercase tracking-[0.18em] text-slate">Storage</p>
          <h2 className="mt-2 text-3xl font-semibold">Category breakdown</h2>
        </div>
        <button
          onClick={() => void scan()}
          className="rounded-md border border-pine/30 px-4 py-2 text-sm hover:bg-mist"
        >
          {loading ? "Scanning…" : "Rescan"}
        </button>
      </header>

      {!summary ? (
        <p className="text-slate">Run Analyze from the dashboard first.</p>
      ) : (
        <ul className="divide-y divide-pine/10 border-y border-pine/10">
          {summary.categories.map((c: CategoryBreakdown) => (
            <li key={c.category} className="flex items-center justify-between gap-4 py-4">
              <div>
                <div className="font-medium capitalize">{c.category.replace(/_/g, " ")}</div>
                <div className="text-sm text-slate">{c.file_count} files</div>
              </div>
              <div className="flex items-center gap-3">
                <span className={`rounded px-2 py-0.5 text-xs ${safetyColor(c.safety)}`}>
                  {safetyLabel(c.safety)}
                </span>
                <span className="tabular-nums font-medium">{formatBytes(c.bytes)}</span>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
