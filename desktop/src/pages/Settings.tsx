import { useEffect, useState } from "react";
import { api } from "../lib/api";

export function SettingsPage() {
  const [info, setInfo] = useState<{
    name: string;
    version: string;
    offline: boolean;
    safety_promise: string;
  }>();
  const [dest, setDest] = useState("");
  const [handshake, setHandshake] = useState<string>("");

  useEffect(() => {
    void (async () => {
      setInfo(await api.appInfo());
      setDest(await api.getBackupDest());
    })();
  }, []);

  return (
    <div className="mx-auto max-w-3xl space-y-8">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Settings</p>
        <h2 className="mt-2 text-3xl font-semibold">Preferences</h2>
      </header>

      {info && (
        <div className="space-y-2 text-sm">
          <div>
            {info.name} v{info.version}
          </div>
          <div className="text-pine">{info.offline ? "Offline mode enforced" : "Online"}</div>
          <p className="text-slate">{info.safety_promise}</p>
        </div>
      )}

      <label className="block text-sm">
        <span className="text-slate">Default backup folder</span>
        <input
          className="mt-1 w-full rounded-md border border-pine/20 bg-white/70 px-3 py-2"
          value={dest}
          onChange={(e) => setDest(e.target.value)}
          onBlur={() => void api.setBackupDest(dest)}
        />
      </label>

      <div>
        <button
          className="rounded-md border border-pine/30 px-4 py-2 text-sm"
          onClick={() =>
            void api.companionDispatch("HANDSHAKE").then((r) => setHandshake(JSON.stringify(r, null, 2)))
          }
        >
          Test companion handshake
        </button>
        {handshake && (
          <pre className="mt-3 overflow-auto rounded-md bg-ink p-3 text-xs text-paper">
            {handshake}
          </pre>
        )}
      </div>
    </div>
  );
}
