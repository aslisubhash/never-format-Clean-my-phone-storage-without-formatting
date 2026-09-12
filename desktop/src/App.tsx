import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import { Shell } from "./components/Shell";
import { AppProvider, useApp } from "./state";
import { DashboardPage } from "./pages/Dashboard";
import { StoragePage } from "./pages/Storage";
import { FilesPage } from "./pages/Files";
import { BackupPage } from "./pages/Backup";
import { MovePage } from "./pages/Move";
import { CleanupPage } from "./pages/Cleanup";
import { WhatsAppPage } from "./pages/WhatsApp";
import { ReportsPage } from "./pages/Reports";
import { DevicePage } from "./pages/Device";
import { SettingsPage } from "./pages/Settings";

function Routed() {
  const { selected } = useApp();
  return (
    <Shell deviceName={selected?.name}>
      <Routes>
        <Route path="/" element={<DashboardPage />} />
        <Route path="/storage" element={<StoragePage />} />
        <Route path="/files" element={<FilesPage />} />
        <Route path="/backup" element={<BackupPage />} />
        <Route path="/move" element={<MovePage />} />
        <Route path="/cleanup" element={<CleanupPage />} />
        <Route path="/whatsapp" element={<WhatsAppPage />} />
        <Route path="/reports" element={<ReportsPage />} />
        <Route path="/device" element={<DevicePage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Shell>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <AppProvider>
        <Routed />
      </AppProvider>
    </BrowserRouter>
  );
}
