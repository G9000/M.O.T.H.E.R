# Memory garden explorer

A read-only way to walk through what M.O.T.H.E.R holds. The place is one painted hill with a great tree on its crown. A footpath leaves the tree and winds down the hill in a spiral, and **the path is time**: the newest record stands where the walk begins, and every step along the path is a step further back, to the first record beside the pool at the path's end.

Each record of the native v1 view is one tall flower beside the path, drawn in flat colour with an inked line so it stands out from the painted meadow.

| Record | Flower |
|---|---|
| Memory | lilac daisy |
| Decision | mint tulip |
| Task | orange poppy |
| Checkpoint | blue bellflower |
| Artifact | sunflower |
| Action | pink rose |
| Pinned (any kind) | a stone lantern in the ring round the start, not on the path |

A flower stands taller the more often its record has been revised. Retracted, superseded and cancelled records are grey and low; tasks in progress or blocked, and actions pending or unknown, slowly pulse. A wooden signpost beside the path marks where each month begins, and the month you are walking through is shown at the bottom left. The path is as long as the records need, so the hill grows with the garden. The garden opens in the gold light of a summer evening and stays there; the lanterns' windows glow, and fireflies come out over the flowers as the sun goes down.

## Open it

```
mother garden explore --db <path to the garden database>
```

The command prints one line of JSON with a `url`, then keeps running until you press Ctrl+C. Open the link in a browser on the same machine. Add `--open` to have the default browser opened for you, and `--limit <1-2000>` to change how many records are shown (default 500: pinned records first, then the most recently updated).

Like the other owner commands that take `--db`, it opens the database directly, so [stop the owner service](garden-service.md) first. It reads one snapshot and closes the database straight away; the service can be started again while the garden stays open. The garden does not refresh: run the command again to see newer records.

The database must have the v1 view enabled (`garden compat-init`). Graph-only objects, proposals waiting in the inbox, the owner persona and credentials are not part of the snapshot.

## Walking

- Click the picture to walk: the mouse looks, `W A S D` move, Shift runs, Space jumps. Esc gives the mouse back.
- Look at a flower within a few steps and press `E`, or click, to read its record: text, status, handoff, next step, linked records and recent revisions. Linked records are buttons that take you to them.
- `L` opens the list of every record, grouped by kind, with a search box; choosing one stands you on the path beside its flower. The list works from the keyboard and needs no walking.
- `T` and `1`–`4` change the time of day, `0` lets the evening pass on to sunset, and `,` and `.` nudge the hour; `P` turns the paint finish off and on.

Nothing can be changed from the garden. Pinning, handoffs and review stay with the existing owner tools.

## Safety

- The server listens on `127.0.0.1` only, on a random port, and serves files embedded in the executable. The page loads nothing from the network; a content security policy enforces that.
- The link carries a single-use token after the `#`. The page trades it once for a session held by that browser tab and removes it from the address bar. A used link, another tab or another site gets no records.
- Requests are refused unless they name the server's own local address, and requests from other origins are refused.
- Only `GET` of the page files and the snapshot, and the one token exchange, are answered.

## Limits

- The page needs WebGL 2 and a capable graphics card. It has been run on one machine only (an RTX 5090); its frame rate with the garden planted was not measured, and integrated graphics, phones, Safari and Firefox are untested. Adding `?quality=low` or `?quality=medium` to the address before the `#` draws less.
- The page is about 2.6 MB and takes several seconds to paint the land and sky on first load.
- Checked with a synthetic garden of 500 records. The record list, the card and standing beside a chosen flower were exercised; walking with the mouse captured was not driven by the automated check.
- Records are ordered by when they were first created. Not carried over from the first version: the time slider, the harness filter and threads drawn between linked flowers. "Similar in meaning" links are still not drawn.
- Long records show their first 4,000 characters, and each record its latest 20 revisions.
- Sound is footsteps only (see `web/garden/MISSING_ASSETS.md`).

## Trying it without real data

```
MOTHER_EXPLORER_DEMO=<new file> cargo test --locked --test garden_explorer demo -- --ignored
```

writes a synthetic garden to a new file (set `MOTHER_EXPLORER_DEMO_RECORDS` to change its size). Point `garden explore --db` at that file. Its records are all made within a second, so they share one month.

## What is ours and what is borrowed

The way the place is drawn is borrowed from [Painted valley](https://patch.gg/blendi/painted-valley) by @blendi on Patch, release 1.1.1 (code MIT, art CC0), on three.js (MIT): the painted meadow and ground, the paint finish over the frame, trees, stones, water, and the sky with its clouds and far mountains. Those files are in `web/garden/src/lib/` and `web/garden/src/valley/`, with the licence files beside them; the same notices are embedded and served with the page. `LICENCE-NOTES.md` and `CREDITS.md` there are the original project's and still list parts this garden no longer ships.

What that project was about is not here: its valley, village, farmland, river, wanderer and physics engine were removed. Ours are the land (`web/garden/src/valley/world.js`: the hill, the spiral path, the pool, the country round it), where trees, bushes and stones stand, the records and everything said about them (`web/garden/src/garden/garden.js`), the page (`web/garden/index.html`), and the hooks in `main.js`. The clouds and the mountain skyline are still the original project's shapes.

## Rebuilding the page

The built page in `assets/garden-explorer/` is committed, so building `mother` needs only Cargo. To change the page:

```
cd web/garden
npm install --ignore-scripts
npm run build
```

then rebuild `mother`. Node is needed for that step only; the executable needs no Node. The file names in `assets/garden-explorer/` are fixed and listed in `src/explorer.rs`; a new file must be added there to be served. The snapshot is `GardenStore::v1_garden_view` in `src/garden/compat_view.rs`.
