import "./global";

const constantify = <T>(path: string): T => null as T;

if (window !== null) {
  window.constantify = constantify;
}

if (globalThis !== null) {
  globalThis.constantify = constantify;
}
