declare global {
  interface Window {
    constantify: <T>(path: string) => T;
  }

  interface GlobalThis {
    constantify: <T>(path: string) => T;
  }
}

export {};
