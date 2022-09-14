use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, BufRead, BufReader, ErrorKind, Read},
    rc::Rc,
    str::{self, FromStr, SplitWhitespace},
};

use fxhash::FxHashMap;

use crate::gfx::{
    BlendMode, Color, Point, ReadSurface, Rect, Scalar, Size, VecSurface, WriteSurface,
};

pub trait Font: Debug {
    fn line_height(&self) -> Scalar;

    fn glyph(&self, c: char) -> Option<&Glyph>;

    fn measure(&self, size: Size, text: &str) -> Size {
        layout(self, Rect::sized(size), text, |_, _, _, _| {})
    }

    fn render(
        &self,
        bounds: Rect,
        text: &str,
        surface: &mut dyn WriteSurface,
        color: Color,
        blend: BlendMode,
    ) {
        layout(self, bounds, text, |src, rect, point, src_color| {
            // TODO: clipping is handled automatically by all surfaces
            //   some form of the surface.blit with a custom blend function would be ideal
            let mut cursor = point;
            for y in rect.top()..=rect.bottom() {
                // clip y
                if cursor.y < bounds.top() {
                    continue;
                }
                if cursor.y > bounds.bottom() {
                    break;
                }
                for x in rect.left()..=rect.right() {
                    // clip x
                    if cursor.x < bounds.left() {
                        continue;
                    }
                    if cursor.x > bounds.right() {
                        break;
                    }
                    let c = src.read((x, y).into());
                    if c == Some(src_color) {
                        surface.write(cursor, color, blend);
                    }
                    cursor.x += 1;
                }
                cursor.y += 1;
                cursor.x = point.x;
            }
        });
    }
}

fn layout<F: Font + ?Sized, O>(font: &F, bounds: Rect, text: &str, mut op: O) -> Size
where
    O: FnMut(&VecSurface, Rect, Point, Color),
{
    // TODO: Support more codepoints
    let line_height = font.line_height();
    if !text.is_ascii() {
        return Size::new(0, line_height);
    }
    let bytes = text.as_bytes();
    let mut cursor = bounds.origin;
    let mut width = 0;

    let mut i = 0;
    while i < bytes.len() {
        let mut j = i;

        // find next word terminator...
        if bytes[j].is_ascii_whitespace() {
            j += 1;
        } else {
            while j < bytes.len() && !bytes[j].is_ascii_whitespace() {
                j += 1;
            }
        }

        // consume the "word" or whatever it is
        let word = &bytes[i..j];
        i += j - i;

        // though it might itself be a newline
        if word[0] == b'\n' {
            cursor.x = bounds.origin.x;
            cursor.y += line_height;
            continue;
        }

        // how wide is the thing?
        let mut word_width = 0;
        for c in word {
            if let Some(glyph) = font.glyph(*c as char) {
                word_width += glyph.width;
            }
        }

        // if it wont fit on this line :'(
        if cursor.x != bounds.origin.x && cursor.x + word_width > bounds.right() {
            // we dont care about trailing spaces though...
            if word[0].is_ascii_whitespace() {
                continue;
            }

            cursor.x = bounds.origin.x;
            cursor.y += line_height;
        }

        // ok time to lay out the word for real
        for c in word {
            if let Some(glyph) = font.glyph(*c as char) {
                let y_offset = line_height - glyph.bbox.height();
                op(
                    &glyph.surface,
                    glyph.bbox,
                    cursor + (glyph.offset.x, y_offset).into(),
                    Color::WHITE,
                );
                cursor.x += glyph.width;
                width = width.max(cursor.x - bounds.left());
            }
        }
    }

    // TODO: off-by-one somewhere???
    (width + 1, line_height + cursor.y).into()
}

#[derive(Debug)]
pub struct Glyph {
    bbox: Rect,
    offset: Point,
    width: Scalar,
    surface: VecSurface,
}

#[derive(Debug)]
pub struct BdfFont {
    line_height: Scalar,
    glyphs: FxHashMap<char, Glyph>,
    measure_cache: Rc<RefCell<FxHashMap<String, (Size, Size)>>>,
}

fn parse_next<T: FromStr>(split: &mut SplitWhitespace) -> io::Result<T> {
    split
        .next()
        .ok_or(ErrorKind::UnexpectedEof)?
        .parse::<T>()
        .map_err(|_| ErrorKind::InvalidData.into())
}

impl BdfFont {
    pub fn new<R: Read>(reader: &mut BufReader<R>) -> io::Result<Self> {
        let mut lines = reader.lines();

        let line = lines.next().ok_or(ErrorKind::UnexpectedEof)??;
        let mut start = line.split_whitespace();
        if start.next().ok_or(ErrorKind::UnexpectedEof)? != "STARTFONT"
            && start.next().ok_or(ErrorKind::UnexpectedEof)? != "2.1"
        {
            return Err(ErrorKind::InvalidData.into());
        }

        let mut default_bbox = Rect::ZERO;
        loop {
            let line = lines.next().ok_or(ErrorKind::UnexpectedEof)??;
            if line.is_empty() {
                continue;
            }
            let mut global = line.split_whitespace();
            match global.next().ok_or(ErrorKind::UnexpectedEof)? {
                "FONTBOUNDINGBOX" => {
                    default_bbox.size.width = parse_next(&mut global)?;
                    default_bbox.size.height = parse_next(&mut global)?;
                    default_bbox.origin.x = parse_next(&mut global)?;
                    default_bbox.origin.y = parse_next(&mut global)?;
                }
                "STARTPROPERTIES" => break,
                _ => {}
            }
        }

        let mut line_height = default_bbox.size.height;
        loop {
            let line = lines.next().ok_or(ErrorKind::UnexpectedEof)??;
            if line.is_empty() {
                continue;
            }
            let mut prop = line.split_whitespace();
            match prop.next().ok_or(ErrorKind::UnexpectedEof)? {
                "PIXEL_SIZE" => {
                    line_height = parse_next(&mut prop)?;
                }
                "CHARS" => break,
                _ => {}
            }
        }

        let mut glyphs = FxHashMap::default();
        'chars: loop {
            let mut c = 0;
            let mut bbox = default_bbox;
            let mut width = bbox.width();
            let mut glyph_surface: Option<VecSurface> = None;
            loop {
                let line = lines.next().ok_or(ErrorKind::UnexpectedEof)??;
                if line.is_empty() {
                    continue;
                }
                let mut data = line.split_whitespace();
                match data.next().ok_or(ErrorKind::UnexpectedEof)? {
                    "ENCODING" => {
                        // The codepoint (may be negative)
                        c = parse_next(&mut data)?;
                    }
                    "DWIDTH" => {
                        width = parse_next(&mut data)?;
                    }
                    "BBX" => {
                        bbox.size.width = parse_next(&mut data)?;
                        bbox.size.height = parse_next(&mut data)?;
                        bbox.origin.x = parse_next(&mut data)?;
                        bbox.origin.y = parse_next(&mut data)?;
                    }
                    "BITMAP" => {
                        // The bitmap section is a series of rows of bytes for the whole char
                        //
                        // In this example the bitmap data is 16-bits (16 pixels) wide:
                        // BITMAP
                        // C000
                        // CDD0
                        // 0000
                        // ENDCHAR
                        glyph_surface = Some(VecSurface::from_size(bbox.size));
                        for y in 0..bbox.height() {
                            let line = lines.next().ok_or(ErrorKind::UnexpectedEof)??;
                            for (byte_x, byte_bytes) in line.as_bytes().chunks_exact(2).enumerate()
                            {
                                let byte_str = str::from_utf8(&byte_bytes)
                                    .map_err(|_| ErrorKind::InvalidData)?;
                                let byte = u8::from_str_radix(byte_str, 16)
                                    .map_err(|_| ErrorKind::InvalidData)?;
                                for bit in 0..8 {
                                    if byte & (0b1000_0000) >> bit != 0 {
                                        glyph_surface
                                            .as_mut()
                                            .ok_or(ErrorKind::InvalidData)?
                                            .write(
                                                ((byte_x as Scalar * 8) + bit, y).into(),
                                                Color::WHITE,
                                                BlendMode::None,
                                            );
                                    }
                                }
                            }
                        }
                    }
                    "ENDCHAR" => {
                        if c >= 0 && c < 256 {
                            glyphs.insert(
                                c as u8 as char,
                                Glyph {
                                    bbox: Rect::sized(bbox.size),
                                    offset: bbox.origin,
                                    width,
                                    surface: glyph_surface.ok_or(ErrorKind::InvalidData)?,
                                },
                            );
                        }
                        break;
                    }
                    "ENDFONT" => break 'chars,
                    _ => {}
                }
            }
        }
        Ok(Self {
            glyphs,
            line_height,
            measure_cache: Rc::new(RefCell::new(FxHashMap::default())),
        })
    }
}

impl Font for BdfFont {
    #[inline]
    fn line_height(&self) -> Scalar {
        self.line_height
    }

    #[inline]
    fn glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&c)
    }

    // TODO: LRU cache
    // fn measure(&self, size: Size, text: &str) -> Size {
    //     {
    //         let cache = self.measure_cache.borrow();
    //         if let Some((input, output)) = cache.get(text) {
    //             if *input == size {
    //                 return *output;
    //             }
    //         }
    //     }
    //     let result = layout(self, Rect::sized(size), text, |_, _, _, _| {});
    //     self.measure_cache
    //         .borrow_mut()
    //         .insert(text.to_owned(), (size, result));
    //     result
    // }
}
