import { promises as fs } from "fs";
import path from "path";
import Viewer, { type Claim, type RunData } from "./Viewer";

export default async function Page() {
  const dataDir = path.join(process.cwd(), "data");
  const run: RunData = JSON.parse(
    await fs.readFile(path.join(dataDir, "run.json"), "utf-8")
  );
  const report = await fs.readFile(path.join(dataDir, "report.md"), "utf-8");
  const lines = report.split("\n");
  const byLine = new Map<number, Claim[]>();
  for (const claim of run.claims) {
    const list = byLine.get(claim.line) ?? [];
    list.push(claim);
    byLine.set(claim.line, list);
  }
  return <Viewer run={run} lines={lines} byLine={Object.fromEntries(byLine)} />;
}
