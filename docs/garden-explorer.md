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

Each record's flower stands in a small clump of smaller blooms of its own kind, away from the path, so the path runs between full borders. A flower stands taller the more often its record has been revised. Retracted, superseded and cancelled records are grey and low; tasks in progress or blocked, and actions pending or unknown, slowly pulse. A wooden signpost beside the path marks where each month begins, and the month you are walking through is shown at the bottom left. The path is as long as the records need, so the hill grows with the garden. The garden opens in the gold light of a summer evening and stays there; the lanterns' windows glow, and fireflies come out over the flowers as the sun goes down.

## Open it

```
mother garden explore --db <path to the garden database>
```

The command prints one line of JSON with a `url`, then keeps running until you press Ctrl+C. Open the link in a browser on the same machine. Add `--open` to have the default browser opened for you, and `--limit <1-2000>` to change how many records are shown (default 500: pinned records first, then the most recently updated).

Like the other owner commands that take `--db`, it opens the database directly, so [stop the owner service](garden-service.md) first. It reads one snapshot and closes the database straight away; the service can be started again while the garden stays open. The garden does not refresh: run the command again to see newer records.

The database must have the v1 view enabled (`garden compat-init`). Other graph-only objects, proposals waiting in the inbox, the owner persona and credentials are not part of the snapshot.

Attached photos, recordings and videos from [visual memory](visual-memory.md) are read in the same visit, with the same rules as the `--media` gallery: stripped copies are shown where they exist, originals carrying embedded location or device data are withheld unless you add `--originals`, and a snapshot holds at most 256 MiB of them. The command's JSON line reports `images` and `withheld`. If the store's media can't be read, the garden opens without them and says so on stderr.

Use the [Living Memory Graph](living-memory-graph.md) to follow directed relationships and review source evidence for an authorized graph representation of a memory.

## Walking

- Click the picture to walk: the mouse looks, `W A S D` move, Shift runs, Space jumps. Esc gives the mouse back.
- Look at a flower within a few steps and press `E`, or click, to read its record: text, status, handoff, next step, linked records and recent revisions. Linked records are buttons that take you to them.
- `/` (or the small lens at the top right) opens the search box. Every word must appear in a record's title, text, status or kind. Found flowers are lit and a soft light hangs over each, the rest fade and shrink back into the meadow, and Enter stands you beside each found flower in turn (Shift+Enter goes back). Esc clears the search.
- A record's card shows the records it names and those that name it; threads of light run from its flower to theirs while the card is open.
- The kinds are a column of coloured dots at the left edge; hover it to open the legend and filter: click a kind to set it aside or bring it back. The harness menu keeps only records that harness wrote or revised, and "Set aside retired records" does what it says. Set-aside flowers stay where they are, faded.
- The timeline at the bottom (faint until you hover it) shows the garden as it stood on any day: later flowers are not yet grown. ▶ replays the garden growing to today.
- The map at the bottom right (larger while you hover it) shows the hill from above: the path, every record as a dot and where you stand, facing which way. Click a dot to walk to that record.
- **The photo grove:** each attached photo, recording and video stands on a wooden easel in a ring round the pool at the path's end, its picture facing the water. A video shows a frame from its first seconds, and a recording a painted stand-in. Look at one and press E, or click, to open the viewer: the item large, its captions, descriptions, OCR text or transcripts with who wrote them, its date, file and source, which copy is shown, and how many memories cite it. Audio and video play there. The arrow keys step through the grove, and Esc closes it. Search finds photos by their words too, the map shows them as small white squares, and the list has them at the top.
- `L` opens the list of every record, grouped by kind; it follows the search. Choosing one stands you on the path beside its flower. The list works from the keyboard and needs no walking.
- The panels hide while you walk with the mouse captured; Esc brings them back. The record count appears when you hover the title.
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
- Records are ordered by when they were first created. Threads are drawn for the open record only, not for the whole garden at once. "Similar in meaning" links are still not drawn.
- Long records show their first 4,000 characters, and each record its latest 20 revisions.
- Sound is footsteps only (see `web/garden/MISSING_ASSETS.md`).

## Trying it without real data

To work on the page itself, run `npm run dev` in `web/garden` and open the address it prints: in the development server only, the page fills itself with invented records (`?records=<n>` changes how many). The production build never contains them.


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

For event dates, evidence and historical comparisons in native Garden records, use the [Life Timeline](life-timeline.md).
