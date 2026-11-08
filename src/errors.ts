export class MotherError extends Error {
  constructor(public code: string, message: string) { super(`${code}: ${message}`); this.name = 'MotherError'; }
}
export function fail(code: string, message: string): never { throw new MotherError(code, message); }
