import { useEffect, useState } from "react";
import { api } from "../lib/api";
import { formatBytes, safetyColor, safetyLabel, type ScannedFile } from "../lib/types";

export function FilesPage() {
  const [files, setFiles] = useState<ScannedFile[]>([]);
  const [large, setLarge] = useState<ScannedFile[]>([]);

  useEffect(() => {
    void (async () => {
      setFiles(await api.listFiles());
      setLarge(await api.largeFiles(50));
    })();
  }, []);

  return (
    <div className="mx-auto max-w-5xl space-y-10">
      <header>
        <p className="text-sm uppercase tracking-[0.18em] text-slate">Files</p>
        <h2 className="mt-2 text-3xl font-semibold">Scanned items</h2>
      </header>

      <section>
        <h3 className="mb-3 text-lg font-medium">Large files (≥ 50 MB)</h3>
        <FileTable files={large} />
      </section>

      <section>
        <h3 className="mb-3 text-lg font-medium">All scanned</h3>
        <FileTable files={files} />
      </section>
    </div>
  );
}

function FileTable({ files }: { files: ScannedFile[] }) {
  if (!files.length) return <p className="text-sm text-slate">No files yet — scan a device.</p>;
  return (
    <div className="overflow-hidden rounded-md border border-pine/15">
      <table className="w-full text-left text-sm">
        <thead className="bg-mist/80 text-slate">
          <tr>
            <th className="px-3 py-2 font-medium">Name</th>
            <th className="px-3 py-2 font-medium">Category</th>
            <th className="px-3 py-2 font-medium">Safety</th>
            <th className="px-3 py-2 font-medium text-right">Size</th>
          </tr>
        </thead>
        <tbody>
          {files.map((f) => (
            <tr key={f.path} className="border-t border-pine/10">
              <td className="px-3 py-2">
                <div className="font-medium">{f.name}</div>
                <div className="truncate text-xs text-slate">{f.path}</div>
              </td>
              <td className="px-3 py-2 capitalize">{f.category.replace(/_/g, " ")}</td>
              <td className="px-3 py-2">
                <span className={`rounded px-2 py-0.5 text-xs ${safetyColor(f.safety)}`}>
                  {safetyLabel(f.safety)}
                </span>
              </td>
              <td className="px-3 py-2 text-right tabular-nums">{formatBytes(f.size_bytes)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
