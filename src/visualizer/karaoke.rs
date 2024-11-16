use super::WINDOW_SIZE;
use crate::r#move::Move;
use kiss3d::{
    nalgebra::{Point2, Point3},
    text::Font,
    window::Window,
};

const TEXT_SCALE: f32 = 100.0;

fn display_size(text: &str) -> f32 {
    text.chars()
        .map(|c| {
            Font::default()
                .font()
                .glyph(c)
                .scaled(rusttype::Scale::uniform(TEXT_SCALE))
                .h_metrics()
                .advance_width
        })
        .sum()
}

pub fn draw_karaoke(text: &str, moves_done: usize, window: &mut Window) {
    let font = Font::default();

    let mut space_count: usize = 0;
    let mut idx = 0;
    for (i, c) in text.char_indices() {
        if c.is_whitespace() {
            space_count += 1;
        }
        if space_count == moves_done {
            idx = i;
            break;
        }
    }
    if space_count < moves_done {
        idx = text.len();
    }

    let cur_line = text[..idx].chars().filter(|&c| c == '\n').count();
    let vmetrics = font.font().v_metrics(rusttype::Scale::uniform(TEXT_SCALE));
    let line_height = vmetrics.ascent - vmetrics.descent;
    let mut char_sum = 0;

    text.lines().enumerate().for_each(|(i, line)| {
        let starty = i as f32 * line_height;
        let centerx = ((WINDOW_SIZE * 2) as f32 - display_size(line)) / 2.0;
        if i == cur_line {
            window.draw_text(
                &line[..idx - char_sum],
                &Point2::new(centerx, starty),
                TEXT_SCALE,
                &font,
                &Point3::new(0.0, 1.0, 0.0),
            );

            let startx = display_size(&line[..idx - char_sum]);

            window.draw_text(
                &line[idx - char_sum..],
                &Point2::new(centerx + startx, starty),
                TEXT_SCALE,
                &font,
                &Point3::new(1.0, 0.0, 0.0),
            );
        } else {
            let color = if i < cur_line {
                Point3::new(0.0, 1.0, 0.0)
            } else {
                Point3::new(1.0, 0.0, 0.0)
            };
            window.draw_text(
                &line,
                &Point2::new(centerx, starty),
                TEXT_SCALE,
                &font,
                &color,
            );
            char_sum += line.chars().count() + 1;
        }
    });
}

pub fn karaoke_format(moves: &[Move]) -> String {
    let mut chars_width = 0.0;

    moves
        .iter()
        .enumerate()
        .map(|(i, &move_)| {
            let mut move_str = format!("{:?}", move_);
            if i > 0 {
                move_str.insert(0, ' ');
            }
            let mut move_display_size = display_size(&move_str);
            if chars_width + move_display_size > (WINDOW_SIZE * 2 - 5) as f32 {
                move_str.insert(0, '\n');
                if move_str[1..].starts_with(" ") {
                    move_str.remove(1);
                    move_display_size = display_size(&move_str);
                }
                chars_width = 0.0;
            }
            chars_width += move_display_size;
            move_str
        })
        .collect()
}
