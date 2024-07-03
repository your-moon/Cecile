"use client";

import { NextUIProvider } from "@nextui-org/react";
import { ThemeProvider as NextThemesProvider } from "next-themes";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <NextUIProvider className="w-full h-full">
      <main className="light text-foreground bg-background w-full h-full">
        {children}
      </main>
    </NextUIProvider>
  );
}
