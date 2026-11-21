//! Pure, bounded image inspection: format sniffing, dimensions, embedded metadata
//! discovery and lossless metadata removal. Nothing here decodes pixels, touches the
//! filesystem or contacts a network service.

/// Image formats accepted as first-class visual evidence.
pub const MEDIA_TYPES: [&str; 7] = [
    "image/jpeg",
    "image/png",
    "image/webp",
    "image/gif",
    "image/heic",
    "image/heif",
    "image/avif",
];

/// Facts derived from the bytes themselves; a declared type is never trusted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inspection {
    pub media_type: &'static str,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// Sorted, de-duplicated metadata categories such as `gps`, `device` or `xmp`.
    pub embedded: Vec<&'static str>,
    /// Whether [`strip`] can produce a metadata-free derivative of this format.
    pub strippable: bool,
}

type Result<T> = std::result::Result<T, &'static str>;
const MALFORMED: &str = "Malformed or truncated image";

fn be16(b: &[u8], at: usize) -> Result<u16> {
    let s = b.get(at..at + 2).ok_or(MALFORMED)?;
    Ok(u16::from_be_bytes([s[0], s[1]]))
}
fn be32(b: &[u8], at: usize) -> Result<u32> {
    let s = b.get(at..at + 4).ok_or(MALFORMED)?;
    Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}
fn le16(b: &[u8], at: usize) -> Result<u16> {
    let s = b.get(at..at + 2).ok_or(MALFORMED)?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}
fn le32(b: &[u8], at: usize) -> Result<u32> {
    let s = b.get(at..at + 4).ok_or(MALFORMED)?;
    Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// Identify a supported image format from its leading bytes.
pub fn sniff(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("image/png");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.len() >= 16 && &bytes[4..8] == b"ftyp" {
        let size = be32(bytes, 0).ok()? as usize;
        let end = size.clamp(16, bytes.len().min(4096));
        let brands: Vec<&[u8]> = std::iter::once(&bytes[8..12])
            .chain(bytes[16..end].chunks_exact(4))
            .collect();
        let has = |names: &[&[u8]]| brands.iter().any(|b| names.contains(b));
        if has(&[b"avif", b"avis"]) {
            return Some("image/avif");
        }
        if has(&[
            b"heic", b"heix", b"hevc", b"hevx", b"heim", b"heis", b"hevm", b"hevs",
        ]) {
            return Some("image/heic");
        }
        if has(&[b"mif1", b"msf1"]) {
            return Some("image/heif");
        }
    }
    None
}

/// Inspect supported image bytes without decoding pixels.
/// # Errors
/// Rejects unknown formats and structurally malformed containers.
pub fn inspect(bytes: &[u8]) -> Result<Inspection> {
    let media_type = sniff(bytes).ok_or("Unsupported or unrecognized image format")?;
    let mut found = Vec::new();
    let (width, height, strippable) = match media_type {
        "image/jpeg" => {
            let dims = jpeg(bytes, &mut found, None)?;
            (dims.map(|d| d.0), dims.map(|d| d.1), true)
        }
        "image/png" => {
            let dims = png(bytes, &mut found, None)?;
            (Some(dims.0), Some(dims.1), true)
        }
        "image/webp" => {
            let dims = webp(bytes, &mut found, None)?;
            (dims.map(|d| d.0), dims.map(|d| d.1), true)
        }
        "image/gif" => {
            let dims = gif(bytes, &mut found, None)?;
            (Some(dims.0), Some(dims.1), true)
        }
        _ => {
            let dims = bmff(bytes, &mut found);
            (dims.map(|d| d.0), dims.map(|d| d.1), false)
        }
    };
    found.sort_unstable();
    found.dedup();
    Ok(Inspection {
        media_type,
        width,
        height,
        embedded: found,
        strippable,
    })
}

/// Produce a derivative without embedded metadata, preserving the encoded image data.
/// JPEG orientation stored in EXIF is removed with the rest of the EXIF block.
/// # Errors
/// Rejects unsupported formats (HEIC, HEIF and AVIF) and malformed input.
pub fn strip(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut ignored = Vec::new();
    match sniff(bytes) {
        Some("image/jpeg") => {
            jpeg(bytes, &mut ignored, Some(&mut out))?;
        }
        Some("image/png") => {
            png(bytes, &mut ignored, Some(&mut out))?;
        }
        Some("image/webp") => {
            webp(bytes, &mut ignored, Some(&mut out))?;
        }
        Some("image/gif") => {
            gif(bytes, &mut ignored, Some(&mut out))?;
        }
        Some(_) => return Err("Metadata removal is not supported for HEIC, HEIF or AVIF"),
        None => return Err("Unsupported or unrecognized image format"),
    }
    Ok(out)
}

/// Standard padded base64 for bounded MCP image content.
pub fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16)
            | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8)
            | u32::from(*chunk.get(2).unwrap_or(&0));
        for (index, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            if index <= chunk.len() {
                out.push(ALPHABET[((n >> shift) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// Classify a TIFF/EXIF block by the privacy-relevant tags it carries.
fn exif(tiff: &[u8], found: &mut Vec<&'static str>) {
    found.push("exif");
    let little = match tiff.get(..2) {
        Some(b"II") => true,
        Some(b"MM") => false,
        _ => return,
    };
    let u16_at = |at| {
        if little {
            le16(tiff, at)
        } else {
            be16(tiff, at)
        }
    };
    let u32_at = |at| {
        if little {
            le32(tiff, at)
        } else {
            be32(tiff, at)
        }
    };
    if u16_at(2) != Ok(42) {
        return;
    }
    let mut ifds = vec![(u32_at(4).unwrap_or(0) as usize, true)];
    let mut visited = 0;
    while let Some((offset, root)) = ifds.pop() {
        visited += 1;
        if offset == 0 || visited > 4 {
            continue;
        }
        let Ok(count) = u16_at(offset) else { continue };
        for entry in 0..usize::from(count.min(512)) {
            let at = offset + 2 + entry * 12;
            let Ok(tag) = u16_at(at) else { break };
            match tag {
                0x8825 => found.push("gps"),
                0x010F | 0x0110 | 0xA431 | 0xA433 | 0xA434 | 0xA435 => found.push("device"),
                0x0132 | 0x9003 | 0x9004 => found.push("timestamp"),
                0x8769 if root => {
                    if let Ok(next) = u32_at(at + 8) {
                        ifds.push((next as usize, false));
                    }
                }
                _ => {}
            }
        }
    }
}

type Dims = (u32, u32);

/// Walk JPEG segments; with `out`, copy everything except metadata segments.
fn jpeg(
    b: &[u8],
    found: &mut Vec<&'static str>,
    mut out: Option<&mut Vec<u8>>,
) -> Result<Option<Dims>> {
    let emit = |out: &mut Option<&mut Vec<u8>>, s: &[u8]| {
        if let Some(out) = out.as_deref_mut() {
            out.extend_from_slice(s);
        }
    };
    emit(&mut out, &b[..2]);
    let mut at = 2;
    let mut dims = None;
    loop {
        // Skip fill bytes before a marker.
        while b.get(at) == Some(&0xFF) && b.get(at + 1) == Some(&0xFF) {
            at += 1;
        }
        if b.get(at) != Some(&0xFF) {
            return Err(MALFORMED);
        }
        let marker = *b.get(at + 1).ok_or(MALFORMED)?;
        if marker == 0xD9 {
            emit(&mut out, &b[at..at + 2]);
            if at + 2 < b.len() {
                found.push("trailer");
            }
            return Ok(dims);
        }
        if (0xD0..=0xD7).contains(&marker) || marker == 0x01 {
            emit(&mut out, &b[at..at + 2]);
            at += 2;
            continue;
        }
        let length = usize::from(be16(b, at + 2)?);
        if length < 2 {
            return Err(MALFORMED);
        }
        let end = at + 2 + length;
        let segment = b.get(at..end).ok_or(MALFORMED)?;
        let body = &segment[4..];
        let keep = match marker {
            0xE1 if body.starts_with(b"Exif\0\0") => {
                exif(&body[6..], found);
                false
            }
            0xE1 if body.starts_with(b"http://ns.adobe.com/xap/1.0/") => {
                found.push("xmp");
                false
            }
            0xE1 => {
                found.push("application");
                false
            }
            0xED => {
                found.push("iptc");
                false
            }
            0xFE => {
                found.push("comment");
                false
            }
            // JFIF, ICC profiles and the Adobe colour transform affect rendering.
            0xE0 | 0xE2 | 0xEE => true,
            0xE3..=0xEF => {
                found.push("application");
                false
            }
            _ => true,
        };
        if matches!(marker, 0xC0..=0xCF) && !matches!(marker, 0xC4 | 0xC8 | 0xCC) {
            dims = Some((u32::from(be16(b, at + 7)?), u32::from(be16(b, at + 5)?)));
        }
        if keep {
            emit(&mut out, segment);
        }
        at = end;
        if marker == 0xDA {
            // Entropy-coded data runs to the next real marker.
            let start = at;
            loop {
                let byte = *b.get(at).ok_or(MALFORMED)?;
                if byte == 0xFF {
                    let next = *b.get(at + 1).ok_or(MALFORMED)?;
                    if next != 0x00 && !(0xD0..=0xD7).contains(&next) && next != 0xFF {
                        break;
                    }
                }
                at += 1;
            }
            emit(&mut out, &b[start..at]);
        }
    }
}

fn png(b: &[u8], found: &mut Vec<&'static str>, mut out: Option<&mut Vec<u8>>) -> Result<Dims> {
    if let Some(out) = out.as_deref_mut() {
        out.extend_from_slice(&b[..8]);
    }
    let mut at = 8;
    let mut dims = None;
    while at < b.len() {
        let length = be32(b, at)? as usize;
        let kind = b.get(at + 4..at + 8).ok_or(MALFORMED)?;
        let end = at
            .checked_add(12)
            .and_then(|v| v.checked_add(length))
            .ok_or(MALFORMED)?;
        let chunk = b.get(at..end).ok_or(MALFORMED)?;
        let data = &chunk[8..8 + length];
        let keep = match kind {
            b"IHDR" => {
                dims = Some((be32(data, 0)?, be32(data, 4)?));
                true
            }
            b"eXIf" => {
                exif(data, found);
                false
            }
            b"iTXt" if data.starts_with(b"XML:com.adobe.xmp\0") => {
                found.push("xmp");
                false
            }
            b"tEXt" | b"zTXt" | b"iTXt" => {
                found.push("text");
                false
            }
            b"tIME" => {
                found.push("timestamp");
                false
            }
            _ => true,
        };
        if keep && let Some(out) = out.as_deref_mut() {
            out.extend_from_slice(chunk);
        }
        at = end;
        if kind == b"IEND" {
            if at < b.len() {
                found.push("trailer");
            }
            return dims.ok_or(MALFORMED);
        }
    }
    Err(MALFORMED)
}

fn webp(
    b: &[u8],
    found: &mut Vec<&'static str>,
    out: Option<&mut Vec<u8>>,
) -> Result<Option<Dims>> {
    let declared = le32(b, 4)? as usize;
    let end = declared.checked_add(8).ok_or(MALFORMED)?;
    if end > b.len() {
        return Err(MALFORMED);
    }
    if end < b.len() {
        found.push("trailer");
    }
    let mut at = 12;
    let mut dims = None;
    let mut kept: Vec<u8> = Vec::new();
    let mut vp8x = None;
    while at < end {
        let kind = b.get(at..at + 4).ok_or(MALFORMED)?;
        let size = le32(b, at + 4)? as usize;
        let padded = size + (size & 1);
        let next = at
            .checked_add(8)
            .and_then(|v| v.checked_add(padded))
            .ok_or(MALFORMED)?;
        if next > end {
            return Err(MALFORMED);
        }
        let data = &b[at + 8..at + 8 + size];
        let keep = match kind {
            b"EXIF" => {
                exif(data.strip_prefix(b"Exif\0\0").unwrap_or(data), found);
                false
            }
            b"XMP " => {
                found.push("xmp");
                false
            }
            b"VP8X" => {
                if data.len() < 10 {
                    return Err(MALFORMED);
                }
                let w = u32::from_le_bytes([data[4], data[5], data[6], 0]) + 1;
                let h = u32::from_le_bytes([data[7], data[8], data[9], 0]) + 1;
                dims = Some((w, h));
                vp8x = Some(kept.len() + 8);
                true
            }
            b"VP8 " if dims.is_none() => {
                if data.len() >= 10 && data[3..6] == [0x9D, 0x01, 0x2A] {
                    dims = Some((
                        u32::from(le16(data, 6)? & 0x3FFF),
                        u32::from(le16(data, 8)? & 0x3FFF),
                    ));
                }
                true
            }
            b"VP8L" if dims.is_none() => {
                if data.len() >= 5 && data[0] == 0x2F {
                    let bits = le32(data, 1)?;
                    dims = Some(((bits & 0x3FFF) + 1, ((bits >> 14) & 0x3FFF) + 1));
                }
                true
            }
            _ => true,
        };
        if keep {
            kept.extend_from_slice(&b[at..next]);
        }
        at = next;
    }
    if let Some(out) = out {
        if let Some(flags) = vp8x {
            // Clear the EXIF and XMP presence bits of the extended header.
            kept[flags] &= !0x0C;
        }
        out.extend_from_slice(b"RIFF");
        let size = u32::try_from(kept.len() + 4).map_err(|_| MALFORMED)?;
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(b"WEBP");
        out.extend_from_slice(&kept);
    }
    Ok(dims)
}

fn sub_blocks(b: &[u8], mut at: usize) -> Result<usize> {
    loop {
        let size = usize::from(*b.get(at).ok_or(MALFORMED)?);
        at += 1 + size;
        if size == 0 {
            return Ok(at);
        }
    }
}

fn gif(b: &[u8], found: &mut Vec<&'static str>, mut out: Option<&mut Vec<u8>>) -> Result<Dims> {
    let dims = (u32::from(le16(b, 6)?), u32::from(le16(b, 8)?));
    let packed = *b.get(10).ok_or(MALFORMED)?;
    let mut at = 13;
    if packed & 0x80 != 0 {
        at += 3 << ((packed & 7) + 1);
    }
    let header = b.get(..at).ok_or(MALFORMED)?;
    if let Some(out) = out.as_deref_mut() {
        out.extend_from_slice(header);
    }
    loop {
        let start = at;
        let (keep, end) = match *b.get(at).ok_or(MALFORMED)? {
            0x3B => {
                if let Some(out) = out.as_deref_mut() {
                    out.push(0x3B);
                }
                if at + 1 < b.len() {
                    found.push("trailer");
                }
                return Ok(dims);
            }
            0x21 => {
                let label = *b.get(at + 1).ok_or(MALFORMED)?;
                let end = sub_blocks(b, at + 2)?;
                let keep = match label {
                    0xFE => {
                        found.push("comment");
                        false
                    }
                    0xFF => {
                        let id = b.get(at + 3..at + 14).ok_or(MALFORMED)?;
                        if id == b"XMP DataXMP" {
                            found.push("xmp");
                            false
                        } else if id == b"NETSCAPE2.0" || id == b"ANIMEXTS1.0" {
                            true
                        } else {
                            found.push("application");
                            false
                        }
                    }
                    _ => true,
                };
                (keep, end)
            }
            0x2C => {
                let local = *b.get(at + 9).ok_or(MALFORMED)?;
                let mut next = at + 10;
                if local & 0x80 != 0 {
                    next += 3 << ((local & 7) + 1);
                }
                (true, sub_blocks(b, next + 1)?)
            }
            _ => return Err(MALFORMED),
        };
        let block = b.get(start..end).ok_or(MALFORMED)?;
        if keep && let Some(out) = out.as_deref_mut() {
            out.extend_from_slice(block);
        }
        at = end;
    }
}

/// Approximate metadata and dimensions for ISO-BMFF images; never used for rewriting.
fn bmff(b: &[u8], found: &mut Vec<&'static str>) -> Option<Dims> {
    let head = &b[..b.len().min(4 * 1024 * 1024)];
    let contains = |needle: &[u8]| head.windows(needle.len()).any(|w| w == needle);
    if contains(b"Exif") {
        found.push("exif");
    }
    if contains(b"application/rdf+xml") {
        found.push("xmp");
    }
    let at = head.windows(4).position(|w| w == b"ispe")?;
    Some((be32(head, at + 8).ok()?, be32(head, at + 12).ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exif_block(tags: &[u16]) -> Vec<u8> {
        let mut t = b"II*\0\x08\0\0\0".to_vec();
        t.extend_from_slice(&(tags.len() as u16).to_le_bytes());
        for tag in tags {
            t.extend_from_slice(&tag.to_le_bytes());
            t.extend_from_slice(&[2, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
        }
        t.extend_from_slice(&[0; 4]);
        t
    }
    pub(crate) fn jpeg_with_gps() -> Vec<u8> {
        let mut app1 = b"Exif\0\0".to_vec();
        app1.extend(exif_block(&[0x010F, 0x8825]));
        let mut b = vec![0xFF, 0xD8, 0xFF, 0xE0, 0, 16];
        b.extend_from_slice(b"JFIF\0\x01\x01\0\0\x01\0\x01\0\0");
        b.extend_from_slice(&[0xFF, 0xE1]);
        b.extend_from_slice(&((app1.len() + 2) as u16).to_be_bytes());
        b.extend(app1);
        b.extend_from_slice(&[0xFF, 0xFE, 0, 6, b'h', b'i', b'!', b'!']);
        b.extend_from_slice(&[0xFF, 0xC0, 0, 11, 8, 0, 3, 0, 4, 1, 1, 0x11, 0]);
        b.extend_from_slice(&[0xFF, 0xDA, 0, 8, 1, 1, 0, 0, 0x3F, 0]);
        b.extend_from_slice(&[0x12, 0xFF, 0x00, 0x34, 0xFF, 0xD0, 0x56]);
        b.extend_from_slice(&[0xFF, 0xD9]);
        b
    }
    fn png_with_text() -> Vec<u8> {
        let mut b = b"\x89PNG\r\n\x1a\n".to_vec();
        let chunk = |b: &mut Vec<u8>, kind: &[u8], data: &[u8]| {
            b.extend_from_slice(&(data.len() as u32).to_be_bytes());
            b.extend_from_slice(kind);
            b.extend_from_slice(data);
            b.extend_from_slice(&[0; 4]);
        };
        chunk(&mut b, b"IHDR", &[0, 0, 0, 5, 0, 0, 0, 7, 8, 2, 0, 0, 0]);
        chunk(&mut b, b"tEXt", b"Author\0synthetic");
        chunk(&mut b, b"eXIf", &exif_block(&[0x8825]));
        chunk(&mut b, b"IDAT", &[1, 2, 3]);
        chunk(&mut b, b"IEND", &[]);
        b
    }

    #[test]
    fn sniffs_formats_and_rejects_others() {
        assert_eq!(sniff(&jpeg_with_gps()), Some("image/jpeg"));
        assert_eq!(sniff(&png_with_text()), Some("image/png"));
        assert_eq!(sniff(b"GIF89a\x01\0\x01\0\0\0\0;"), Some("image/gif"));
        assert_eq!(
            sniff(b"\0\0\0\x18ftypheic\0\0\0\0mif1heic"),
            Some("image/heic")
        );
        assert_eq!(
            sniff(b"\0\0\0\x18ftypmif1\0\0\0\0mif1avif"),
            Some("image/avif")
        );
        assert_eq!(sniff(b"%PDF-1.7"), None);
        assert!(inspect(b"plain text").is_err());
    }
    #[test]
    fn jpeg_metadata_is_reported_and_removed_losslessly() {
        let original = jpeg_with_gps();
        let found = inspect(&original).unwrap();
        assert_eq!((found.width, found.height), (Some(4), Some(3)));
        assert_eq!(found.embedded, ["comment", "device", "exif", "gps"]);
        let stripped = strip(&original).unwrap();
        let clean = inspect(&stripped).unwrap();
        assert!(clean.embedded.is_empty());
        assert_eq!((clean.width, clean.height), (Some(4), Some(3)));
        // The entropy-coded scan, including stuffed bytes and restart markers, is intact.
        assert!(stripped.ends_with(&[0x12, 0xFF, 0x00, 0x34, 0xFF, 0xD0, 0x56, 0xFF, 0xD9]));
        assert_ne!(original, stripped);
    }
    #[test]
    fn png_text_and_exif_are_removed() {
        let found = inspect(&png_with_text()).unwrap();
        assert_eq!((found.width, found.height), (Some(5), Some(7)));
        assert_eq!(found.embedded, ["exif", "gps", "text"]);
        let clean = inspect(&strip(&png_with_text()).unwrap()).unwrap();
        assert!(clean.embedded.is_empty());
    }
    #[test]
    fn webp_and_gif_metadata_are_removed() {
        let mut chunks = Vec::new();
        chunks.extend_from_slice(b"VP8X\x0a\0\0\0\x0c\0\0\0\x01\0\0\x02\0\0");
        chunks.extend_from_slice(b"EXIF\x04\0\0\0abcd");
        chunks.extend_from_slice(b"XMP \x03\0\0\0xyz\0");
        let mut webp = b"RIFF".to_vec();
        webp.extend_from_slice(&((chunks.len() + 4) as u32).to_le_bytes());
        webp.extend_from_slice(b"WEBP");
        webp.extend(chunks);
        let found = inspect(&webp).unwrap();
        assert_eq!((found.width, found.height), (Some(2), Some(3)));
        assert_eq!(found.embedded, ["exif", "xmp"]);
        let stripped = strip(&webp).unwrap();
        assert!(inspect(&stripped).unwrap().embedded.is_empty());
        assert_eq!(stripped[20] & 0x0C, 0);

        let mut gif = b"GIF89a\x02\0\x02\0\0\0\0".to_vec();
        gif.extend_from_slice(b"\x21\xfe\x03abc\0");
        gif.extend_from_slice(b"\x21\xff\x0bXMP DataXMP\x01x\0");
        gif.extend_from_slice(b"\x2c\0\0\0\0\x02\0\x02\0\0\x02\x02\x44\x01\0;");
        let found = inspect(&gif).unwrap();
        assert_eq!(found.embedded, ["comment", "xmp"]);
        assert!(inspect(&strip(&gif).unwrap()).unwrap().embedded.is_empty());
    }
    #[test]
    fn heif_family_is_reported_but_not_rewritten() {
        let heic = b"\0\0\0\x18ftypheic\0\0\0\0mif1heicExif....ispe\0\0\0\0\0\0\0\x09\0\0\0\x0b";
        let found = inspect(heic).unwrap();
        assert!(!found.strippable);
        assert_eq!(found.embedded, ["exif"]);
        assert_eq!((found.width, found.height), (Some(9), Some(11)));
        assert!(strip(heic).is_err());
    }
    #[test]
    fn truncated_input_is_rejected() {
        let jpeg = jpeg_with_gps();
        assert!(inspect(&jpeg[..jpeg.len() - 9]).is_err());
        let png = png_with_text();
        assert!(inspect(&png[..png.len() - 4]).is_err());
    }
    #[test]
    fn base64_matches_reference_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
