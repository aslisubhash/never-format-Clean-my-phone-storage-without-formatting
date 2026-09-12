import { createContext, useContext, useEffect, useState, useCallback } from "react";
import { api } from "./lib/api";
import type { DeviceInfo, StorageSummary } from "./lib/types";

interface AppCtx {
  devices: DeviceInfo[];
  selected?: DeviceInfo;
  summary?: StorageSummary;
  loading: boolean;
  error?: string;
  refreshDevices: () => Promise<void>;
  selectDevice: (id: string) => void;
  scan: () => Promise<void>;
}

const Ctx = createContext<AppCtx | null>(null);

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [selectedId, setSelectedId] = useState<string>();
  const [summary, setSummary] = useState<StorageSummary>();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>();

  const refreshDevices = useCallback(async () => {
    setLoading(true);
    setError(undefined);
    try {
      const list = await api.listDevices();
      setDevices(list);
      if (!selectedId && list[0]) setSelectedId(list[0].id);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [selectedId]);

  const scan = useCallback(async () => {
    if (!selectedId) return;
    setLoading(true);
    setError(undefined);
    try {
      const s = await api.scanDevice(selectedId);
      setSummary(s);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [selectedId]);

  useEffect(() => {
    void refreshDevices();
  }, []);

  const selected = devices.find((d) => d.id === selectedId);

  return (
    <Ctx.Provider
      value={{
        devices,
        selected,
        summary,
        loading,
        error,
        refreshDevices,
        selectDevice: setSelectedId,
        scan,
      }}
    >
      {children}
    </Ctx.Provider>
  );
}

export function useApp() {
  const v = useContext(Ctx);
  if (!v) throw new Error("AppProvider missing");
  return v;
}
