const ignored = new Set('a an and are at by can day did do find for from i in it me moment my of on please recall remember show that the this to was we when where with you'.split(' '));
const words = (s) => String(s ?? '').toLowerCase().match(/[\p{L}\p{N}]+/gu) ?? [];

export function rankMoments(records, clue, includeKeepsakes = false) {
  const terms = [...new Set(words(clue).filter((word) => !ignored.has(word)))];
  return records.filter((r) => !r.sourceReview && (includeKeepsakes || !r.keepsake) && !['retracted', 'superseded', 'cancelled'].includes(r.status)).map((record) => {
    const title = new Set(words(record.title));
    const content = new Set(words(record.content));
    const matched = terms.filter((term) => title.has(term) || content.has(term));
    return {record, matched, titleMatches: matched.filter((t) => title.has(t)).length};
  }).filter((hit) => hit.matched.length).sort((a, b) =>
    b.matched.length - a.matched.length || b.titleMatches - a.titleMatches ||
    String(b.record.updatedAt ?? '').localeCompare(String(a.record.updatedAt ?? '')) || String(a.record.id).localeCompare(String(b.record.id)));
}
