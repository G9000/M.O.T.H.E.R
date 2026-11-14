//! The approved artwork rendered with ordinary colored terminal cells.
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

const SIDE: usize = 64;
const PIXELS: &[u8; SIDE * SIDE * 4] = include_bytes!("../../assets/mother-logo.rgba");

pub(super) struct Logo {
    pub background: (u8, u8, u8),
}

impl Logo {
    /// Average premultiplied pixels so transparent edges do not acquire dark fringes.
    fn sample(&self, x: usize, y: usize, size: usize) -> Color {
        let mut channels = [0_u32; 3];
        let background = [self.background.0, self.background.1, self.background.2];
        let mut count = 0;
        for sy in y * SIDE / size..(y + 1) * SIDE / size {
            for sx in x * SIDE / size..(x + 1) * SIDE / size {
                let pixel = &PIXELS[(sy * SIDE + sx) * 4..][..4];
                let alpha = u32::from(pixel[3]);
                for channel in 0..3 {
                    channels[channel] += u32::from(pixel[channel]) * alpha
                        + u32::from(background[channel]) * (255 - alpha);
                }
                count += 1;
            }
        }
        let divisor = count * 255;
        let [r, g, b] = channels.map(|value| ((value + divisor / 2) / divisor) as u8);
        Color::Rgb(r, g, b)
    }
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // A cell is roughly twice as tall as it is wide. Two pixels per cell
        // retain the square artwork's proportions without any image protocol.
        let area = area.intersection(buf.area);
        let size = area
            .width
            .min(area.height.saturating_mul(2))
            .min(SIDE as u16)
            & !1;
        if size == 0 {
            return;
        }
        let left = area.x + (area.width - size) / 2;
        let top = area.y + (area.height - size / 2) / 2;
        for y in 0..size / 2 {
            for x in 0..size {
                buf[(left + x, top + y)]
                    .set_char('▀')
                    .set_fg(self.sample(usize::from(x), usize::from(y * 2), usize::from(size)))
                    .set_bg(self.sample(usize::from(x), usize::from(y * 2 + 1), usize::from(size)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparency_blends_into_the_terminal_background() {
        let logo = Logo {
            background: (25, 23, 32),
        };
        assert_eq!(logo.sample(0, 0, SIDE), Color::Rgb(25, 23, 32));
        assert_ne!(logo.sample(32, 32, SIDE), Color::Rgb(25, 23, 32));
    }

    #[test]
    fn logo_clips_safely_and_leaves_surrounding_cells_untouched() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        for area in [
            Rect::new(0, 0, 0, 0),
            Rect::new(1, 1, 1, 1),
            Rect::new(8, 3, 30, 20),
        ] {
            Logo {
                background: (25, 23, 32),
            }
            .render(area, &mut buffer);
        }
        assert_eq!(buffer[(0, 0)].symbol(), " ");
        assert_eq!(buffer[(8, 3)].symbol(), "▀");
    }
}
