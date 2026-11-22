"use client";

import { useMemo, useState } from "react";

export interface Claim {
  id: string;
  line: number;
  start: number;
  end: number;
  class: string;
  trust: "trusted" | "provisional";
  value: string;
  status: "verified" | "contradicted" | "unverifiable";
  evidence: { source: string; detail: string };
}

export interface RunData {
  generated_at: string;
  generator: string;
  title: string;
  limit_line: string;
  claims: Claim[];
  summary: {
    claims: number;
    verified: number;
    provisional: number;
    contradicted: number;
    unverifiable: number;
  };
}

const CLASS_LABELS: Record<string, string> = {
  gitsha: "Full commit SHA",
  commit: "Short commit reference",
  hash256: "Content hash",
  tally: "Run tally",
  pr: "PR reference",
};

function verdictLabel(claim: Claim): string {
  if (claim.status === "verified" && claim.trust === "provisional")
    return "VERIFIED · PROVISIONAL";
  if (claim.status === "verified") return "VERIFIED";
  if (claim.status === "contradicted") return "CONTRADICTED";
  return "UNVERIFIABLE";
}

function verdictClass(claim: Claim): string {
  if (claim.status === "verified" && claim.trust === "provisional")
    return "provisional";
  return claim.status;
}

const WHY: Record<string, string> = {
  verified:
    "The evidence in the bundle recomputes to exactly what the report claims.",
  contradicted:
    "The evidence in the bundle disagrees with the report. The run fails.",
  unverifiable:
    "The evidence this claim needs is not in the bundle, so the claim cannot be checked. Fail closed: the run fails.",
};

export default function Viewer({
  run,
  lines,
  byLine,
}: {
  run: RunData;
  lines: string[];
  byLine: Record<number, Claim[]>;
}) {
  const [selectedId, setSelectedId] = useState(run.claims[0]?.id ?? "");
  const selected = useMemo(
    () => run.claims.find((c) => c.id === selectedId) ?? run.claims[0],
    [run.claims, selectedId]
  );
  const s = run.summary;

  return (
    <main className="shell">
      <header className="masthead">
        <div>
          <div className="wordmark">
            CROSSCHECK<span className="wordmark-sub">verdict viewer</span>
          </div>
          <p className="tagline">
            One real agent work report, checked claim by claim against its
            evidence bundle. Every verdict below was recomputed from the
            evidence, not written by hand.
          </p>
        </div>
        <div className="runmeta">
          <div>
            recorded run · {run.generated_at}
          </div>
          <div className="mono">{run.generator}</div>
        </div>
      </header>

      <section className="summarybar">
        <div className="stat">
          <span className="stat-num">{s.claims}</span> claims extracted
        </div>
        <div className="stat ok">
          <span className="stat-num">{s.verified}</span> verified
        </div>
        <div className="stat prov">
          <span className="stat-num">{s.provisional}</span> verified,
          provisional class
        </div>
        <div className="stat bad">
          <span className="stat-num">{s.contradicted}</span> contradicted
        </div>
        <div className="stat unk">
          <span className="stat-num">{s.unverifiable}</span> unverifiable
        </div>
        <div className="exitbox">
          run exit code <strong>1</strong> — a contradicted claim fails the
          run
        </div>
      </section>

      <div className="grid">
        <section className="report pane">
          <div className="pane-head">
            <span>report.md</span>
            <span className="pane-head-note">
              the milestone report of the verifier seat, 2026-11-22
            </span>
          </div>
          <div className="report-body">
            {lines.map((line, idx) => {
              const lineNo = idx + 1;
              const claims = (byLine[lineNo] ?? [])
                .slice()
                .sort((a, b) => a.start - b.start);
              const isH1 = line.startsWith("# ");
              const isH2 = line.startsWith("## ");
              const content = renderLine(line, claims, selectedId, setSelectedId);
              if (isH1) return <h1 key={lineNo} className="r-h1">{content}</h1>;
              if (isH2) return <h2 key={lineNo} className="r-h2">{content}</h2>;
              if (line.trim() === "") return <div key={lineNo} className="r-blank" />;
              return (
                <p key={lineNo} className="r-line">
                  <span className="lineno">{lineNo}</span>
                  <span>{content}</span>
                </p>
              );
            })}
          </div>
        </section>

        <aside className="inspector">
          {selected && (
            <section className="pane claim-card">
              <div className="pane-head">
                <span>claim {selected.id}</span>
                <span className="pane-head-note">line {selected.line}</span>
              </div>
              <div className={`verdict ${verdictClass(selected)}`}>
                {verdictLabel(selected)}
              </div>
              <div className="claim-value mono">{selected.value}</div>
              <dl className="facts">
                <dt>class</dt>
                <dd>
                  {CLASS_LABELS[selected.class] ?? selected.class}
                  {selected.trust === "provisional" &&
                    " — provisional: measured at 3.7% false positives in M1, never fails a run alone"}
                </dd>
                <dt>evidence</dt>
                <dd>{selected.evidence.source}</dd>
                <dt>what the check found</dt>
                <dd>{selected.evidence.detail}</dd>
                <dt>why this verdict</dt>
                <dd>{WHY[selected.status]}</dd>
              </dl>
            </section>
          )}

          <section className="pane claim-list">
            <div className="pane-head">
              <span>all claims</span>
              <span className="pane-head-note">{run.claims.length}</span>
            </div>
            <ul>
              {run.claims.map((claim) => (
                <li key={claim.id}>
                  <button
                    className={`claim-row ${claim.id === selectedId ? "active" : ""}`}
                    onClick={() => setSelectedId(claim.id)}
                  >
                    <span className={`dot ${verdictClass(claim)}`} />
                    <span className="mono claim-row-value">{claim.value}</span>
                    <span className="claim-row-class">
                      {CLASS_LABELS[claim.class] ?? claim.class}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          </section>
        </aside>
      </div>

      <footer className="foot">
        <p className="limit">{run.limit_line}</p>
        <p>
          Demo viewer, built as showcase material. The product is the
          deterministic CLI in this repository; this page renders one
          recorded run over a real report and the evidence bundle it cites
          (the repository&apos;s own git history, the M1 record on disk,
          the Phase 1 prototype, and host records fetched from the GitHub
          API on 2026-11-22). The report carries one planted error and one
          claim whose evidence is deliberately outside the bundle, so the
          run shows every verdict the tool can return.
        </p>
      </footer>
    </main>
  );
}

function renderLine(
  line: string,
  claims: Claim[],
  selectedId: string,
  setSelectedId: (id: string) => void
) {
  if (claims.length === 0) return line;
  const parts: React.ReactNode[] = [];
  let cursor = 0;
  claims.forEach((claim, i) => {
    if (claim.start > cursor) parts.push(line.slice(cursor, claim.start));
    parts.push(
      <button
        key={`${claim.id}-${i}`}
        className={`span ${verdictClass(claim)} ${claim.id === selectedId ? "active" : ""}`}
        onClick={() => setSelectedId(claim.id)}
        title={`${verdictLabel(claim)} — ${CLASS_LABELS[claim.class] ?? claim.class}`}
      >
        {line.slice(claim.start, claim.end)}
      </button>
    );
    cursor = claim.end;
  });
  if (cursor < line.length) parts.push(line.slice(cursor));
  return parts;
}
