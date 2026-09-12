import { useEffect, useState } from "react";
import { api } from "../lib/api";
import type { Operation, Report } from "../lib/types";

export function ReportsPage() {
  const [ops, setOps] = useState<Operation[]>([]);
  const [reports, setReports] = useState<Report[]>([]);

  useEffect(() => {
    void (async () => {
      setOps(await api.listOperations());
      setReports(await api.listReports());
    })();
  }, []);

  return (
    <div className="mx-auto max-w-4xl space-y-10">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Reports</p>
        <h2 className="mt-2 text-3xl font-semibold">History & reports</h2>
      </header>

      <section>
        <h3 className="mb-3 font-medium">Operations</h3>
        <ul className="space-y-2 text-sm">
          {ops.map((o) => (
            <li key={o.id} className="rounded-md border border-pine/15 px-3 py-2">
              <div className="font-medium">
                {o.kind} — {o.state}
              </div>
              <div className="text-slate">{o.message}</div>
            </li>
          ))}
          {!ops.length && <li className="text-slate">No operations yet.</li>}
        </ul>
      </section>

      <section>
        <h3 className="mb-3 font-medium">Reports</h3>
        <ul className="space-y-3">
          {reports.map((r) => (
            <li key={r.id} className="rounded-md border border-pine/15 p-3">
              <div className="font-medium">
                {r.success ? "✓" : "✗"} {r.title}
              </div>
              <pre className="mt-2 max-h-40 overflow-auto text-xs text-slate">{r.body}</pre>
            </li>
          ))}
          {!reports.length && <li className="text-slate">No reports yet.</li>}
        </ul>
      </section>
    </div>
  );
}
