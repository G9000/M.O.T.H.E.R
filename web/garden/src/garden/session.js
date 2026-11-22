// The page's one conversation with the local server that served it: a session traded for the launch link, then
// read-only requests for the snapshot, its media and single records. Nothing else is sent, and to nowhere else.
/**
 * The link from the terminal carries a single-use token after the "#". It is traded once for this tab's session
 * and taken out of the address bar; a reload uses the session again.
 */
let session = null;
export async function loadSnapshot() {
  try {
    const launch = location.hash.slice(1);
    try {
      session = sessionStorage.getItem('garden-session');
    } catch {
      // storage may be blocked: this page still has the session it is about to be given
    }
    if (launch) {
      history.replaceState(null, '', location.pathname + location.search);
      const res = await fetch('./api/session', { method: 'POST', headers: { Authorization: `Bearer ${launch}` } });
      if (res.ok) {
        session = (await res.json()).session;
        try {
          sessionStorage.setItem('garden-session', session);
        } catch {
          // as above
        }
      }
    }
    return await fetchSnapshot();
  } catch {
    return null;
  }
}

/** One record's full reading (its whole text, provenance and links), or null when it cannot be had. */
export async function fetchReading(id) {
  if (!session) return null;
  const res = await fetch(`./api/garden/records/${encodeURIComponent(id)}`, { headers: { Authorization: `Bearer ${session}` } });
  return res.ok ? res.json() : null;
}

export async function fetchSnapshot(since) {
  if (!session) return null;
  const path = since ? `./api/garden?since=${encodeURIComponent(since)}` : './api/garden';
  const res = await fetch(path, { headers: { Authorization: `Bearer ${session}` } });
  return res.ok ? res.json() : null;
}

/** The attached photos, recordings and videos of the same snapshot, each with its bytes: [{ item, blob }]. */
export async function loadMedia() {
  if (!session) return [];
  try {
    const auth = { headers: { Authorization: `Bearer ${session}` } };
    const res = await fetch('./api/media', auth);
    if (!res.ok) return [];
    const { images = [] } = await res.json();
    const out = await Promise.all(
      images.map(async (item) => {
        const r = await fetch(`./api/media/${item.sha256}`, auth).catch(() => null);
        return r?.ok ? { item, blob: await r.blob() } : null;
      }),
    );
    return out.filter(Boolean);
  } catch {
    return [];
  }
}

