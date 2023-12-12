// placeholder noop, utilized to avoid lint issues only
const constant = {
  bool: () => true,
  enum: () => ({}),
  number: () => 0,
  str: () => "",
};

if (window !== null) {
  window.constant = constant;
}

if (globalThis !== null) {
  globalThis.constant = constant;
}
