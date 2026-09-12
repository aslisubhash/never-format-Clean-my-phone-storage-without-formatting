import { NavLink } from "react-router-dom";
import {
  LayoutDashboard,
  HardDrive,
  Files,
  Archive,
  ArrowRightLeft,
  Trash2,
  MessageCircle,
  FileText,
  Smartphone,
  Settings,
} from "lucide-react";

const links = [
  { to: "/", label: "Dashboard", icon: LayoutDashboard },
  { to: "/storage", label: "Storage", icon: HardDrive },
  { to: "/files", label: "Files", icon: Files },
  { to: "/backup", label: "Backup", icon: Archive },
  { to: "/move", label: "Move", icon: ArrowRightLeft },
  { to: "/cleanup", label: "Cleanup", icon: Trash2 },
  { to: "/whatsapp", label: "WhatsApp", icon: MessageCircle },
  { to: "/reports", label: "Reports", icon: FileText },
  { to: "/device", label: "Device", icon: Smartphone },
  { to: "/settings", label: "Settings", icon: Settings },
];

export function Shell({
  children,
  deviceName,
}: {
  children: React.ReactNode;
  deviceName?: string;
}) {
  return (
    <div className="flex h-full min-h-0">
      <aside className="flex w-56 shrink-0 flex-col border-r border-pine/10 bg-ink text-paper">
        <div className="border-b border-white/10 px-5 py-6">
          <div className="text-xs uppercase tracking-[0.2em] text-leaf">Offline</div>
          <h1 className="mt-1 text-2xl font-semibold tracking-tight">Never Format</h1>
          <p className="mt-2 text-xs leading-relaxed text-white/60">
            Analyze → Protect → Verify → Clean
          </p>
        </div>
        <nav className="flex-1 space-y-0.5 overflow-y-auto p-3">
          {links.map(({ to, label, icon: Icon }) => (
            <NavLink
              key={to}
              to={to}
              end={to === "/"}
              className={({ isActive }) =>
                `flex items-center gap-3 rounded-md px-3 py-2 text-sm transition ${
                  isActive
                    ? "bg-pine text-white"
                    : "text-white/75 hover:bg-white/5 hover:text-white"
                }`
              }
            >
              <Icon size={16} strokeWidth={1.75} />
              {label}
            </NavLink>
          ))}
        </nav>
        <div className="border-t border-white/10 px-4 py-3 text-xs text-white/50">
          {deviceName ? `Connected: ${deviceName}` : "No device"}
        </div>
      </aside>
      <main className="min-w-0 flex-1 overflow-y-auto p-8">{children}</main>
    </div>
  );
}
