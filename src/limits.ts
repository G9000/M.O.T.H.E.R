import { fail } from './errors';
export const MAX_EXPORT_BYTES = 64 * 1024 * 1024;
export interface Usage { records: number; operations: number; payloadBytes: number }
// Compact JSON export adds operation envelopes and one deduplicated source-URL list.
// Doubling stored payload bytes bounds that URL list; fixed allowances cover wrappers
// and the bounded persona/metadata. Keep admission below the portable file limit.
export function checkUsage(usage: Usage) {
  const exportBound = 2 * usage.payloadBytes + 512 * usage.operations + 128 * usage.records + 262144;
  if (usage.records > 10000 || usage.operations > 100000 || exportBound > MAX_EXPORT_BYTES) fail('RESOURCE_LIMIT', 'Workspace reached its portable capacity. Export it before archival or migration; this write was not committed.');
}
