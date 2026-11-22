// Local, read-only gallery of one owner snapshot. Every request goes back to the
// loopback server that served this page; nothing is sent anywhere else.
"use strict";
const status = document.getElementById("status");
const grid = document.getElementById("images");
const viewer = document.getElementById("viewer");
const launch = location.hash.slice(1);
history.replaceState(null, "", location.pathname);

const ORIGIN = { captured: "Taken", uploaded: "Uploaded", imported: "Imported", screenshot: "Screenshot", ai_generated: "Made by AI" };
const READING = { caption: "Caption", description: "Description", ocr_text: "Text in the image", transcript: "Transcript" };
const esc = (s) => String(s ?? "").replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);
const items = [];
let at = -1;

function describe(image, url) {
  const kind = image.mediaType.split("/")[0];
  const first = image.acquisitions?.[0] ?? {};
  const readings = (image.interpretations ?? []).map((r) => ({
    label: READING[String(r.predicate ?? "").replace("mother.media:", "")] ?? "Note",
    text: String(r.text ?? ""),
    by: r.attribution?.producer?.name ?? r.contributor ?? "",
  }));
  const caption = readings.find((r) => r.label === "Caption")?.text;
  const when = Date.parse(first.eventTime?.at ?? first.acquiredAt ?? "") || 0;
  return {
    kind,
    url,
    title: caption || first.originalFilename || (kind === "audio" ? "A recording" : kind === "video" ? "A video" : "A photo"),
    file: first.originalFilename ?? "",
    origin: ORIGIN[first.origin] ?? "",
    source: first.source ?? "",
    when: when ? new Date(when).toLocaleDateString(undefined, { day: "numeric", month: "long", year: "numeric" }) : "",
    readings,
    variant: image.variant,
    uses: image.uses ?? 0,
  };
}

function show(i) {
  at = (i + items.length) % items.length;
  const p = items[at];
  const media = viewer.querySelector(".v-media");
  media.innerHTML = "";
  const el = document.createElement(p.kind === "image" ? "img" : p.kind);
  el.src = p.url;
  if (p.kind === "image") el.alt = p.title;
  else {
    el.controls = true;
    el.autoplay = true;
  }
  media.append(el);
  viewer.querySelector(".v-text").innerHTML = `
    <span class="v-kind">${p.kind === "image" ? "Photo" : p.kind === "audio" ? "Recording" : "Video"}${p.origin ? ` · ${esc(p.origin)}` : ""}</span>
    <h2>${esc(p.title)}</h2>
    ${p.when ? `<p class="v-when">${esc(p.when)}</p>` : ""}
    ${p.readings.map((r) => `<div class="v-read"><b>${esc(r.label)}</b><p>${esc(r.text)}</p>${r.by ? `<span>by ${esc(r.by)}</span>` : ""}</div>`).join("")}
    <dl>
      ${p.file ? `<dt>File</dt><dd>${esc(p.file)}</dd>` : ""}
      ${p.source ? `<dt>Source</dt><dd>${esc(p.source)}</dd>` : ""}
      <dt>Shown</dt><dd>${p.variant === "derivative" ? "A copy with location and device data removed" : "The original"}</dd>
      <dt>Cited by</dt><dd>${p.uses === 1 ? "1 memory" : `${esc(p.uses)} memories`}</dd>
    </dl>
    ${items.length > 1 ? `<p class="v-count">${at + 1} of ${items.length}</p>` : ""}`;
  viewer.querySelector(".v-prev").hidden = viewer.querySelector(".v-next").hidden = items.length < 2;
  viewer.hidden = false;
}
function close() {
  for (const el of viewer.querySelectorAll("audio, video")) el.pause();
  viewer.hidden = true;
}
viewer.addEventListener("click", (e) => {
  const step = e.target.closest("[data-step]");
  if (step) show(at + Number(step.dataset.step));
  else if (e.target.closest("[data-close]")) close();
});
document.addEventListener("keydown", (e) => {
  if (viewer.hidden) return;
  if (e.key === "Escape") close();
  else if (e.key === "ArrowRight") show(at + 1);
  else if (e.key === "ArrowLeft") show(at - 1);
});

async function main() {
  const opened = await fetch("/api/session", {
    method: "POST",
    headers: { Authorization: "Bearer " + launch },
  });
  if (!opened.ok) {
    status.textContent = "This link has already been used. Run the command again for a new one.";
    return;
  }
  const { session } = await opened.json();
  const auth = { headers: { Authorization: "Bearer " + session } };
  const list = await (await fetch("/api/media", auth)).json();
  const withheld = list.withheld
    ? ` · ${list.withheld} withheld for embedded location or device data (--originals shows them)`
    : "";
  const skipped = list.truncated ? " · some large recordings were left out" : "";
  status.textContent = `${list.images.length} ${list.images.length === 1 ? "item" : "items"}${withheld}${skipped}`;
  if (!list.images.length) {
    grid.outerHTML = '<p class="empty">No attached photos, recordings or videos yet.</p>';
    return;
  }
  for (const image of list.images) {
    const reply = await fetch("/api/media/" + image.sha256, auth);
    if (!reply.ok) continue;
    const item = describe(image, URL.createObjectURL(await reply.blob()));
    const index = items.push(item) - 1;
    const tile = document.createElement("button");
    tile.className = `tile ${item.kind}`;
    tile.style.animationDelay = `${Math.min(index, 20) * 40}ms`;
    let face;
    if (item.kind === "image") {
      face = document.createElement("img");
      face.alt = item.title;
      face.loading = "lazy";
      face.src = item.url;
    } else if (item.kind === "video") {
      face = document.createElement("video");
      face.muted = true;
      face.preload = "metadata";
      face.src = item.url + "#t=1";
    } else {
      face = document.createElement("div");
      face.className = "stand-in";
      face.textContent = "♪";
    }
    const meta = document.createElement("div");
    meta.className = "meta";
    meta.innerHTML = `<b>${esc(item.title)}</b><span>${esc([item.when, item.origin].filter(Boolean).join(" · "))}</span>`;
    tile.append(face, meta);
    if (item.kind !== "image") {
      const badge = document.createElement("span");
      badge.className = "badge";
      badge.textContent = item.kind === "audio" ? "Recording" : "Video";
      tile.append(badge);
    }
    tile.addEventListener("click", () => show(index));
    grid.append(tile);
  }
}
main().catch(() => {
  status.textContent = "The gallery could not be loaded.";
});
