import { useApp } from "../state";
import { formatBytes } from "../lib/types";

export function DevicePage() {
  const { devices, selected, selectDevice, refreshDevices, loading } = useApp();

  return (
    <div className="mx-auto max-w-4xl space-y-8">
      <header className="flex items-end justify-between">
        <div>
          <p className="text-sm uppercase tracking-[0.18em] text-slate">Device</p>
          <h2 className="mt-2 text-3xl font-semibold">Connection</h2>
        </div>
        <button
          onClick={() => void refreshDevices()}
          className="rounded-md border border-pine/30 px-4 py-2 text-sm"
        >
          {loading ? "Refreshing…" : "Refresh"}
        </button>
      </header>

      <div className="rounded-md border border-pine/20 bg-mist/50 px-4 py-3 text-sm text-slate">
        <strong className="text-ink">USB debugging required</strong> for full Android analysis.
        Never Format does not upload files. Enable Developer options → USB debugging, then
        reconnect.
      </div>

      <ul className="space-y-3">
        {devices.map((d) => (
          <li key={d.id}>
            <button
              onClick={() => selectDevice(d.id)}
              className={`w-full rounded-md border px-4 py-4 text-left transition ${
                selected?.id === d.id
                  ? "border-pine bg-white"
                  : "border-pine/15 hover:border-pine/40"
              }`}
            >
              <div className="font-medium">{d.name}</div>
              <div className="mt-1 text-sm text-slate">
                {d.platform} · {d.transport} · {d.serial}
              </div>
              <div className="mt-2 text-sm">
                {formatBytes(d.used_bytes)} / {formatBytes(d.total_bytes)} ·{" "}
                {formatBytes(d.free_bytes)} free
              </div>
              <div className="mt-2 flex flex-wrap gap-1">
                {d.capabilities.map((c: string) => (
                  <span key={c} className="rounded bg-mist px-2 py-0.5 text-xs text-slate">
                    {c}
                  </span>
                ))}
              </div>
            </button>
          </li>
        ))}
        {!devices.length && <li className="text-slate">No devices found.</li>}
      </ul>
    </div>
  );
}
