import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Crosscheck — verdict viewer",
  description:
    "One real agent work report, checked claim by claim against its evidence bundle.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
