// The valley was written as a Patch block and reports its progress to the page that embeds it. Here nothing
// embeds it and nothing is told anything: the same calls, going nowhere.
export function patchBlock() {
  return { progress() {}, ready() {}, state() {} };
}
