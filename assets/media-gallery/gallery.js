// Local, read-only gallery of one owner snapshot. Every request goes back to the
// loopback server that served this page; nothing is sent anywhere else.
"use strict";
const status = document.getElementById("status");
const grid = document.getElementById("images");
const launch = location.hash.slice(1);
history.replaceState(null, "", location.pathname);

function line(label, value) {
  const row = document.createElement("div");
  const strong = document.createElement("strong");
  strong.textContent = label + ": ";
  row.append(strong, document.createTextNode(String(value)));
  return row;
}

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
    ? ` ${list.withheld} with embedded location or device data are withheld (use --originals to show them).`
    : "";
  const skipped = list.truncated ? " Some large recordings were left out of this snapshot." : "";
  status.textContent = `${list.images.length} attached items.${withheld}${skipped}`;
  for (const image of list.images) {
    const reply = await fetch("/api/media/" + image.sha256, auth);
    if (!reply.ok) continue;
    const figure = document.createElement("figure");
    const kind = image.mediaType.split("/")[0];
    const img = document.createElement(kind === "image" ? "img" : kind);
    if (kind === "image") {
      img.alt = image.acquisitions[0]?.originalFilename ?? "Attached image";
    } else {
      img.controls = true;
      img.preload = "metadata";
    }
    img.src = URL.createObjectURL(await reply.blob());
    const caption = document.createElement("figcaption");
    const first = image.acquisitions[0] ?? {};
    if (first.originalFilename) caption.append(line("File", first.originalFilename));
    if (first.eventTime?.at) caption.append(line("Event", first.eventTime.at));
    if (first.origin) caption.append(line("Origin", first.origin.replace("_", " ")));
    caption.append(line("Shown", image.variant));
    caption.append(line("Cited by", `${image.uses} evidence record(s)`));
    figure.append(img, caption);
    grid.append(figure);
  }
}
main().catch(() => {
  status.textContent = "The gallery could not be loaded.";
});
