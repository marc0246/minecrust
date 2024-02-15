#![allow(clippy::identity_op)]

use anyhow::{ensure, Result};
use png::{BitDepth, ColorType};

use super::model::resize_scratch;

pub trait Uploader {
    fn upload(
        &mut self,
        info: Info,
        op: &mut dyn FnMut(&mut [[u8; 4]]) -> Result<()>,
    ) -> Result<()>;
}

#[derive(Clone, Copy)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub index: u32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: All of the following ought to be upstreamed into png as `Transformation`s.

pub(super) fn read(
    reader: &mut png::Reader<crate::File<'_>>,
    upload_buffer: &mut [[u8; 4]],
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let info = reader.info();
    let color_type = info.color_type;
    let buffer_size = info.width as usize * info.height as usize * color_type.samples();

    // If the texture is already in the Blessed Format™, then there's no need for a conversion.
    // This should be the case for some of the textures, but unfortunately most are going to be
    // lacking the alpha channel, needing a conversion. Some textures may even be indexed.
    if color_type == ColorType::Rgba {
        ensure!(upload_buffer.flatten().len() == buffer_size);
        reader.next_frame(upload_buffer.flatten_mut())?;
        return Ok(());
    }

    let scratch = resize_scratch(scratch, buffer_size);
    reader.next_frame(scratch)?;

    let trns = reader.info().trns.as_deref();
    let bit_depth = reader.info().bit_depth;

    match color_type {
        ColorType::Grayscale => {
            ensure!(upload_buffer.len() == scratch.len());

            if let Some(trns) = trns {
                ensure!(trns.len() == 1);
                read_grayscale_trns(upload_buffer, scratch, trns[0]);
            } else {
                match bit_depth {
                    BitDepth::One => read_grayscale1(upload_buffer, scratch),
                    BitDepth::Two => read_grayscale2(upload_buffer, scratch),
                    BitDepth::Four => read_grayscale4(upload_buffer, scratch),
                    BitDepth::Eight => read_grayscale8(upload_buffer, scratch),
                    BitDepth::Sixteen => unreachable!(),
                }
            }
        }
        ColorType::Rgb => {
            let (scratch, _) = scratch.as_chunks();
            ensure!(upload_buffer.len() == scratch.len());

            if let Some(trns) = trns {
                ensure!(trns.len() == 3);
                read_rgb_trns(upload_buffer, scratch, trns.try_into().unwrap());
            } else {
                read_rgb(upload_buffer, scratch);
            }
        }
        ColorType::Indexed => {
            ensure!(upload_buffer.len() == scratch.len());

            let (palette, _) = reader.info().palette.as_deref().unwrap().as_chunks();

            if let Some(trns) = trns {
                match bit_depth {
                    BitDepth::One => read_indexed_trns1(upload_buffer, scratch, palette, trns),
                    BitDepth::Two => read_indexed_trns2(upload_buffer, scratch, palette, trns),
                    BitDepth::Four => read_indexed_trns4(upload_buffer, scratch, palette, trns),
                    BitDepth::Eight => read_indexed_trns8(upload_buffer, scratch, palette, trns),
                    BitDepth::Sixteen => unreachable!(),
                }
            } else {
                match bit_depth {
                    BitDepth::One => read_indexed1(upload_buffer, scratch, palette),
                    BitDepth::Two => read_indexed2(upload_buffer, scratch, palette),
                    BitDepth::Four => read_indexed4(upload_buffer, scratch, palette),
                    BitDepth::Eight => read_indexed8(upload_buffer, scratch, palette),
                    BitDepth::Sixteen => unreachable!(),
                }
            }
        }
        ColorType::GrayscaleAlpha => {
            let (scratch, _) = scratch.as_chunks();
            ensure!(upload_buffer.len() == scratch.len());
            read_grayscale_alpha(upload_buffer, scratch);
        }
        ColorType::Rgba => {
            unreachable!();
        }
    }

    Ok(())
}

fn read_grayscale1(upload_buffer: &mut [[u8; 4]], data: &[u8]) {
    for (output, &l) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let l0 = (l >> 7) & 0x1;
        let l1 = (l >> 6) & 0x1;
        let l2 = (l >> 5) & 0x1;
        let l3 = (l >> 4) & 0x1;
        let l4 = (l >> 3) & 0x1;
        let l5 = (l >> 2) & 0x1;
        let l6 = (l >> 1) & 0x1;
        let l7 = (l >> 0) & 0x1;
        *output = [
            [l0, l0, l0, u8::MAX],
            [l1, l1, l1, u8::MAX],
            [l2, l2, l2, u8::MAX],
            [l3, l3, l3, u8::MAX],
            [l4, l4, l4, u8::MAX],
            [l5, l5, l5, u8::MAX],
            [l6, l6, l6, u8::MAX],
            [l7, l7, l7, u8::MAX],
        ];
    }
}

fn read_grayscale2(upload_buffer: &mut [[u8; 4]], data: &[u8]) {
    for (output, &l) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let l0 = (l >> 6) & 0x3;
        let l1 = (l >> 4) & 0x3;
        let l2 = (l >> 2) & 0x3;
        let l3 = (l >> 0) & 0x3;
        *output = [
            [l0, l0, l0, u8::MAX],
            [l1, l1, l1, u8::MAX],
            [l2, l2, l2, u8::MAX],
            [l3, l3, l3, u8::MAX],
        ];
    }
}

fn read_grayscale4(upload_buffer: &mut [[u8; 4]], data: &[u8]) {
    for (output, &l) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let l0 = (l >> 4) & 0xF;
        let l1 = (l >> 0) & 0xF;
        *output = [[l0, l0, l0, u8::MAX], [l1, l1, l1, u8::MAX]];
    }
}

fn read_grayscale8(upload_buffer: &mut [[u8; 4]], data: &[u8]) {
    for (output, &l) in upload_buffer.iter_mut().zip(data.iter()) {
        *output = [l, l, l, u8::MAX];
    }
}

fn read_grayscale_trns(upload_buffer: &mut [[u8; 4]], data: &[u8], trns: u8) {
    for (output, &l) in upload_buffer.iter_mut().zip(data.iter()) {
        let a = if l == trns { 0 } else { u8::MAX };
        *output = [l, l, l, a];
    }
}

fn read_rgb(upload_buffer: &mut [[u8; 4]], data: &[[u8; 3]]) {
    for (output, &[r, g, b]) in upload_buffer.iter_mut().zip(data.iter()) {
        *output = [r, g, b, u8::MAX];
    }
}

fn read_rgb_trns(upload_buffer: &mut [[u8; 4]], data: &[[u8; 3]], trns: [u8; 3]) {
    for (output, &input @ [r, g, b]) in upload_buffer.iter_mut().zip(data.iter()) {
        let a = if input == trns { 0 } else { u8::MAX };
        *output = [r, g, b, a];
    }
}

fn read_indexed1(upload_buffer: &mut [[u8; 4]], data: &[u8], palette: &[[u8; 3]]) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 7) & 0x1;
        let i1 = (i >> 6) & 0x1;
        let i2 = (i >> 5) & 0x1;
        let i3 = (i >> 4) & 0x1;
        let i4 = (i >> 3) & 0x1;
        let i5 = (i >> 2) & 0x1;
        let i6 = (i >> 1) & 0x1;
        let i7 = (i >> 0) & 0x1;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let [r1, g1, b1] = palette[usize::from(i1)];
        let [r2, g2, b2] = palette[usize::from(i2)];
        let [r3, g3, b3] = palette[usize::from(i3)];
        let [r4, g4, b4] = palette[usize::from(i4)];
        let [r5, g5, b5] = palette[usize::from(i5)];
        let [r6, g6, b6] = palette[usize::from(i6)];
        let [r7, g7, b7] = palette[usize::from(i7)];
        *output = [
            [r0, g0, b0, u8::MAX],
            [r1, g1, b1, u8::MAX],
            [r2, g2, b2, u8::MAX],
            [r3, g3, b3, u8::MAX],
            [r4, g4, b4, u8::MAX],
            [r5, g5, b5, u8::MAX],
            [r6, g6, b6, u8::MAX],
            [r7, g7, b7, u8::MAX],
        ];
    }
}

fn read_indexed2(upload_buffer: &mut [[u8; 4]], data: &[u8], palette: &[[u8; 3]]) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 6) & 0x3;
        let i1 = (i >> 4) & 0x3;
        let i2 = (i >> 2) & 0x3;
        let i3 = (i >> 0) & 0x3;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let [r1, g1, b1] = palette[usize::from(i1)];
        let [r2, g2, b2] = palette[usize::from(i2)];
        let [r3, g3, b3] = palette[usize::from(i3)];
        *output = [
            [r0, g0, b0, u8::MAX],
            [r1, g1, b1, u8::MAX],
            [r2, g2, b2, u8::MAX],
            [r3, g3, b3, u8::MAX],
        ];
    }
}

fn read_indexed4(upload_buffer: &mut [[u8; 4]], data: &[u8], palette: &[[u8; 3]]) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 4) & 0xF;
        let i1 = (i >> 0) & 0xF;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let [r1, g1, b1] = palette[usize::from(i1)];
        *output = [[r0, g0, b0, u8::MAX], [r1, g1, b1, u8::MAX]];
    }
}

fn read_indexed8(upload_buffer: &mut [[u8; 4]], data: &[u8], palette: &[[u8; 3]]) {
    for (output, &i) in upload_buffer.iter_mut().zip(data.iter()) {
        let [r, g, b] = palette[usize::from(i)];
        *output = [r, g, b, u8::MAX];
    }
}

fn read_indexed_trns1(
    upload_buffer: &mut [[u8; 4]],
    data: &[u8],
    palette: &[[u8; 3]],
    trns: &[u8],
) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 7) & 0x1;
        let i1 = (i >> 6) & 0x1;
        let i2 = (i >> 5) & 0x1;
        let i3 = (i >> 4) & 0x1;
        let i4 = (i >> 3) & 0x1;
        let i5 = (i >> 2) & 0x1;
        let i6 = (i >> 1) & 0x1;
        let i7 = (i >> 0) & 0x1;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let &a0 = trns.get(usize::from(i0)).unwrap_or(&u8::MAX);
        let [r1, g1, b1] = palette[usize::from(i1)];
        let &a1 = trns.get(usize::from(i1)).unwrap_or(&u8::MAX);
        let [r2, g2, b2] = palette[usize::from(i2)];
        let &a2 = trns.get(usize::from(i2)).unwrap_or(&u8::MAX);
        let [r3, g3, b3] = palette[usize::from(i3)];
        let &a3 = trns.get(usize::from(i3)).unwrap_or(&u8::MAX);
        let [r4, g4, b4] = palette[usize::from(i4)];
        let &a4 = trns.get(usize::from(i4)).unwrap_or(&u8::MAX);
        let [r5, g5, b5] = palette[usize::from(i5)];
        let &a5 = trns.get(usize::from(i5)).unwrap_or(&u8::MAX);
        let [r6, g6, b6] = palette[usize::from(i6)];
        let &a6 = trns.get(usize::from(i6)).unwrap_or(&u8::MAX);
        let [r7, g7, b7] = palette[usize::from(i7)];
        let &a7 = trns.get(usize::from(i7)).unwrap_or(&u8::MAX);
        *output = [
            [r0, g0, b0, a0],
            [r1, g1, b1, a1],
            [r2, g2, b2, a2],
            [r3, g3, b3, a3],
            [r4, g4, b4, a4],
            [r5, g5, b5, a5],
            [r6, g6, b6, a6],
            [r7, g7, b7, a7],
        ];
    }
}

fn read_indexed_trns2(
    upload_buffer: &mut [[u8; 4]],
    data: &[u8],
    palette: &[[u8; 3]],
    trns: &[u8],
) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 6) & 0x3;
        let i1 = (i >> 4) & 0x3;
        let i2 = (i >> 2) & 0x3;
        let i3 = (i >> 0) & 0x3;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let &a0 = trns.get(usize::from(i0)).unwrap_or(&u8::MAX);
        let [r1, g1, b1] = palette[usize::from(i1)];
        let &a1 = trns.get(usize::from(i1)).unwrap_or(&u8::MAX);
        let [r2, g2, b2] = palette[usize::from(i2)];
        let &a2 = trns.get(usize::from(i2)).unwrap_or(&u8::MAX);
        let [r3, g3, b3] = palette[usize::from(i3)];
        let &a3 = trns.get(usize::from(i3)).unwrap_or(&u8::MAX);
        *output = [
            [r0, g0, b0, a0],
            [r1, g1, b1, a1],
            [r2, g2, b2, a2],
            [r3, g3, b3, a3],
        ];
    }
}

fn read_indexed_trns4(
    upload_buffer: &mut [[u8; 4]],
    data: &[u8],
    palette: &[[u8; 3]],
    trns: &[u8],
) {
    for (output, &i) in upload_buffer.as_chunks_mut().0.iter_mut().zip(data.iter()) {
        let i0 = (i >> 4) & 0xF;
        let i1 = (i >> 0) & 0xF;
        let [r0, g0, b0] = palette[usize::from(i0)];
        let &a0 = trns.get(usize::from(i0)).unwrap_or(&u8::MAX);
        let [r1, g1, b1] = palette[usize::from(i1)];
        let &a1 = trns.get(usize::from(i1)).unwrap_or(&u8::MAX);
        *output = [[r0, g0, b0, a0], [r1, g1, b1, a1]];
    }
}

fn read_indexed_trns8(
    upload_buffer: &mut [[u8; 4]],
    data: &[u8],
    palette: &[[u8; 3]],
    trns: &[u8],
) {
    for (output, &i) in upload_buffer.iter_mut().zip(data.iter()) {
        let [r, g, b] = palette[usize::from(i)];
        let &a = trns.get(usize::from(i)).unwrap_or(&u8::MAX);
        *output = [r, g, b, a];
    }
}

fn read_grayscale_alpha(upload_buffer: &mut [[u8; 4]], data: &[[u8; 2]]) {
    for (output, &[l, a]) in upload_buffer.iter_mut().zip(data.iter()) {
        *output = [l, l, l, a];
    }
}
