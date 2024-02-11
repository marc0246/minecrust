// This module contains many conversions from indices to positions and back, so these lints get old
// *very* quick.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::{fmt, ops};

use anyhow::{anyhow, bail, Context, Error, Result};
use byteorder::{BigEndian, ReadBytesExt};
use enum_map::Enum;
use quartz_nbt::io::{read_nbt, Flavor};
use quartz_nbt::{NbtCompound, NbtList, NbtTag};
use rustc_hash::FxHashMap;
use strum::EnumIter;
pub use world_block::{self as block, Block, BlockId};
pub use world_resource as resource;

#[derive(Default)]
pub struct World {
    save_path: Option<PathBuf>,
    loaded_chunks: FxHashMap<ChunkPos, Chunk>,
    loaded_regions: FxHashMap<RegionPos, File>,
}

impl World {
    /// Size of a region, in bits.
    const REGION_SIZE: usize = 5;

    /// Region coordinate mask.
    const REGION_MASK: i32 = (1 << Self::REGION_SIZE) - 1;

    const REGION_CHUNK_COUNT: usize = 1 << (2 * Self::REGION_SIZE);

    const REGION_SECTOR_SIZE: usize = 4 * Self::REGION_CHUNK_COUNT;

    #[must_use]
    pub fn from_save(save_path: impl AsRef<Path>) -> Self {
        World::new(Some(save_path.as_ref().to_owned()))
    }

    #[must_use]
    pub fn new(save_path: Option<PathBuf>) -> Self {
        World {
            save_path,
            ..Default::default()
        }
    }

    pub fn loaded_chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.loaded_chunks.values()
    }

    #[must_use]
    pub fn loaded_chunk(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        self.loaded_chunks.get(&chunk_pos)
    }

    pub fn load_chunk(&mut self, chunk_pos: ChunkPos) -> Result<()> {
        let region_pos = RegionPos(chunk_pos.0 >> Self::REGION_SIZE as i32);
        let region_chunk_pos = (chunk_pos.0 & Self::REGION_MASK).as_uvec2();
        let chunk_index = (region_chunk_pos.y << Self::REGION_SIZE) + region_chunk_pos.x;

        let region = match self.loaded_regions.entry(region_pos) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let save_path = self
                    .save_path
                    .as_ref()
                    .context("failed to open region file")?;
                let region_path = format!("region/r.{}.{}.mca", region_pos.0.x, region_pos.0.y);
                let path = save_path.join(region_path);
                let file = File::open(path).context("failed to open region file")?;

                entry.insert(file)
            }
        };

        region.seek(SeekFrom::Start(4 * u64::from(chunk_index)))?;
        let location = region.read_u32::<BigEndian>()?;

        if location == 0 {
            log::warn!("Chunk at position {chunk_pos} is not yet generated");
            return Ok(());
        }

        let sector_offset = location >> 8;
        region.seek(SeekFrom::Start(
            u64::from(sector_offset) * Self::REGION_SECTOR_SIZE as u64,
        ))?;

        let _chunk_len = region.read_u32::<BigEndian>()?;
        let flavor = match region.read_u8()? {
            1 => Flavor::GzCompressed,
            2 => Flavor::ZlibCompressed,
            3 => Flavor::Uncompressed,
            n => bail!("unknown compression type `{n}`"),
        };
        let (chunk_tag, _) = read_nbt(region, flavor)?;
        let chunk = Self::read_chunk(&chunk_tag)
            .with_context(|| format!("failed to load chunk at position {chunk_pos}"))?;

        if chunk.pos != chunk_pos {
            bail!("chunk at position {chunk_pos} seems to be corrupted");
        }

        self.loaded_chunks.insert(chunk_pos, chunk);

        Ok(())
    }

    fn read_chunk(chunk_tag: &NbtCompound) -> Result<Chunk> {
        let chunk_pos = ChunkPos(glam::ivec2(
            chunk_tag.get::<_, i32>("xPos")?,
            chunk_tag.get::<_, i32>("zPos")?,
        ));
        let sections_tag = chunk_tag.get::<_, &NbtList>("sections")?;

        if sections_tag.len() != Chunk::SECTION_COUNT {
            log::warn!("Chunk at position {chunk_pos} has missing sections");
        }

        let mut chunk = Chunk::from_block(chunk_pos, Block::from_id(BlockId::Air));

        for (i, section_tag) in sections_tag.iter().enumerate() {
            chunk
                .load_section(section_tag.try_into()?)
                .with_context(|| format!("failed to load section at index `{i}`"))?;
        }

        Ok(chunk)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockPos(glam::IVec3);

impl BlockPos {
    const MIN_Y: i32 = SectionY::MIN << Section::SIZE;

    const MAX_Y: i32 = SectionY::MAX << Section::SIZE;

    pub fn new(pos: glam::IVec3) -> Result<Self> {
        if (Self::MIN_Y..Self::MAX_Y).contains(&pos.y) {
            Ok(BlockPos(pos))
        } else {
            Err(anyhow!("{pos} is outside of the build limit"))
        }
    }

    /// Converts the block position into the position of the chunk the block is in, as well as the
    /// chunk-relative block position within that chunk.
    #[must_use]
    pub fn to_chunk_pos(mut self) -> (ChunkPos, ChunkBlockPos) {
        let chunk_pos = glam::ivec2(self.0.x >> Section::SIZE, self.0.z >> Section::SIZE);
        self.0.x &= Section::MASK;
        self.0.z &= Section::MASK;

        (ChunkPos(chunk_pos), ChunkBlockPos(self.0))
    }

    #[must_use]
    pub fn to_inner(self) -> glam::IVec3 {
        self.0
    }

    #[must_use]
    pub fn neighbor(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Down | Direction::Up => {
                Self::try_from(self.0 + direction.to_unit_vec()).ok()
            }
            _ => Some(BlockPos(self.0 + direction.to_unit_vec())),
        }
    }
}

impl fmt::Display for BlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<glam::IVec3> for BlockPos {
    type Error = anyhow::Error;

    fn try_from(pos: glam::IVec3) -> Result<Self, Self::Error> {
        BlockPos::new(pos)
    }
}

/// Chunk position, in chunks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos(glam::IVec2);

impl ChunkPos {
    #[must_use]
    pub fn new(pos: glam::IVec2) -> Self {
        // All 2D positions are valid chunk positions.
        ChunkPos(pos)
    }
}

impl fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct RegionPos(glam::IVec2);

impl fmt::Display for RegionPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct Chunk {
    pos: ChunkPos,
    sections: [Section; Self::SECTION_COUNT],
}

impl Chunk {
    const SECTION_COUNT: usize = 24;

    #[must_use]
    pub fn from_block(pos: ChunkPos, block: Block) -> Self {
        Chunk {
            pos,
            sections: [Section::from_block(block); Self::SECTION_COUNT],
        }
    }

    #[must_use]
    pub fn pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn blocks(&self) -> impl '_ + Iterator<Item = (BlockPos, Block)> {
        self.sections().flat_map(move |(section_y, section)| {
            section.blocks().map(move |(section_block_pos, block)| {
                let chunk_block_pos = section_block_pos + section_y;

                (chunk_block_pos + self.pos, block)
            })
        })
    }

    fn sections(&self) -> impl Iterator<Item = (SectionY, &Section)> {
        self.sections
            .iter()
            .enumerate()
            .map(|(section_index, section)| (SectionY::from_index(section_index), section))
    }

    fn load_section(&mut self, section_tag: &NbtCompound) -> Result<()> {
        let section_y = section_tag.get::<_, SectionY>("Y")?;
        let block_states_tag = section_tag.get::<_, &NbtCompound>("block_states")?;
        let palette_tag = block_states_tag.get::<_, &NbtList>("palette")?;

        let palette = palette_tag
            .iter()
            .map(|tag| {
                Block::try_from(<&NbtCompound>::try_from(tag)?)
                    .context("failed to read block from its NBT representation")
            })
            .collect::<Result<Vec<Block>, Error>>()?;

        if palette.len() == 1 {
            if palette[0].id() != BlockId::Air {
                self[section_y] = Section::from_block(palette[0]);
            }

            return Ok(());
        }

        let data = block_states_tag.get::<_, &[i64]>("data")?;
        let index_bits = u32::max(usize::BITS - (palette.len() - 1).leading_zeros(), 4);
        let index_packing = i64::BITS / index_bits;
        let index_mask = (1 << index_bits) - 1;

        // DIY up-rounding integer division.
        let data_len = (Section::BLOCK_COUNT + index_packing as usize - 1) / index_packing as usize;

        if data.len() != data_len {
            bail!(
                "invalid length of list in the `data` tag: expected `{}`, found `{}`",
                data_len,
                data.len(),
            );
        }

        for block_index in 0..Section::BLOCK_COUNT {
            let data_index = block_index / index_packing as usize;
            let packed_index = block_index as u32 % index_packing;
            let packed = data[data_index] as u64;
            let palette_index = (packed >> (packed_index * index_bits)) & index_mask;

            self[section_y].blocks[block_index] =
                *palette.get(palette_index as usize).ok_or_else(|| {
                    anyhow!(
                        "palette index `{}` is out of range for palette of length `{}`",
                        palette_index,
                        palette.len(),
                    )
                })?;
        }

        Ok(())
    }
}

impl ops::Index<ChunkBlockPos> for Chunk {
    type Output = Block;

    fn index(&self, pos: ChunkBlockPos) -> &Self::Output {
        let (section_y, section_block_pos) = pos.to_section_pos();

        &self[section_y][section_block_pos]
    }
}

impl ops::IndexMut<ChunkBlockPos> for Chunk {
    fn index_mut(&mut self, pos: ChunkBlockPos) -> &mut Self::Output {
        let (section_y, section_block_pos) = pos.to_section_pos();

        &mut self[section_y][section_block_pos]
    }
}

impl ops::Index<SectionY> for Chunk {
    type Output = Section;

    fn index(&self, pos: SectionY) -> &Self::Output {
        &self.sections[pos.to_index()]
    }
}

impl ops::IndexMut<SectionY> for Chunk {
    fn index_mut(&mut self, pos: SectionY) -> &mut Self::Output {
        &mut self.sections[pos.to_index()]
    }
}

/// Block position relative to a chunk.
#[derive(Clone, Copy, Debug)]
pub struct ChunkBlockPos(glam::IVec3);

impl ChunkBlockPos {
    /// Converts the chunk-relative block position to the section the block is in, as well as the
    /// section-relative block position within that section.
    fn to_section_pos(mut self) -> (SectionY, SectionBlockPos) {
        let section_y = self.0.y >> Section::SIZE;
        self.0.y &= Section::MASK;

        (SectionY(section_y), SectionBlockPos(self.0))
    }
}

impl ops::Add<ChunkPos> for ChunkBlockPos {
    type Output = BlockPos;

    fn add(self, rhs: ChunkPos) -> Self::Output {
        BlockPos(self.0 + glam::ivec3(rhs.0.x, 0, rhs.0.y) * (1 << Section::SIZE))
    }
}

impl fmt::Display for ChunkBlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Section Y position, in sections.
#[derive(Clone, Copy, Debug)]
struct SectionY(i32);

impl SectionY {
    const MIN: i32 = -4;

    const MAX: i32 = Chunk::SECTION_COUNT as i32 + Self::MIN;

    fn new(section_y: i32) -> Result<Self> {
        if (Self::MIN..Self::MAX).contains(&section_y) {
            Ok(SectionY(section_y))
        } else {
            Err(anyhow!("invalid section Y-position `{section_y}`"))
        }
    }

    /// Converts the chunk's section index into the section position.
    fn from_index(index: usize) -> Self {
        SectionY(index as i32 + Self::MIN)
    }

    /// Converts the section position to a chunk section index.
    fn to_index(self) -> usize {
        (self.0 - Self::MIN) as usize
    }
}

impl fmt::Display for SectionY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<&NbtTag> for SectionY {
    type Error = anyhow::Error;

    fn try_from(tag: &NbtTag) -> Result<Self, Self::Error> {
        let section_y = i32::from(i8::try_from(tag)?);

        SectionY::new(section_y)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
struct Section {
    blocks: [Block; Self::BLOCK_COUNT],
}

impl Section {
    /// Size of a section, in bits.
    const SIZE: u32 = 4;

    /// Section coordinate mask.
    const MASK: i32 = (1 << Self::SIZE) - 1;

    const BLOCK_COUNT: usize = 1 << (3 * Self::SIZE);

    fn from_block(block: Block) -> Self {
        Section {
            blocks: [block; Self::BLOCK_COUNT],
        }
    }

    fn blocks(&self) -> impl '_ + Iterator<Item = (SectionBlockPos, Block)> {
        self.blocks
            .iter()
            .enumerate()
            .map(|(block_index, &block)| (SectionBlockPos::from_index(block_index), block))
    }
}

impl ops::Index<SectionBlockPos> for Section {
    type Output = Block;

    fn index(&self, pos: SectionBlockPos) -> &Self::Output {
        &self.blocks[pos.to_index()]
    }
}

impl ops::IndexMut<SectionBlockPos> for Section {
    fn index_mut(&mut self, pos: SectionBlockPos) -> &mut Self::Output {
        &mut self.blocks[pos.to_index()]
    }
}

/// Block position relative to a section.
#[derive(Clone, Copy, Debug)]
struct SectionBlockPos(glam::IVec3);

impl SectionBlockPos {
    /// Converts a section block index to a section-relative block position.
    fn from_index(block_index: usize) -> Self {
        debug_assert!(block_index < i32::MAX as usize);

        let block_index = block_index as i32;
        let y = block_index >> (2 * Section::SIZE);
        let z = (block_index >> Section::SIZE) & Section::MASK;
        let x = block_index & Section::MASK;

        SectionBlockPos(glam::ivec3(x, y, z))
    }

    /// Converts the position into a section block index.
    fn to_index(self) -> usize {
        ((self.0.y << (2 * Section::SIZE)) + (self.0.z << Section::SIZE) + self.0.x) as usize
    }
}

impl fmt::Display for SectionBlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Add<SectionY> for SectionBlockPos {
    type Output = ChunkBlockPos;

    fn add(self, rhs: SectionY) -> Self::Output {
        // Section Y position, in blocks.
        let y_offset = glam::IVec3::Y * (rhs.0 << Section::SIZE);

        ChunkBlockPos(self.0 + y_offset)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Direction along the 3 axes.
#[derive(Clone, Copy, Debug, Enum, EnumIter, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_unit_vec(self) -> glam::IVec3 {
        match self {
            Self::Down  => glam::ivec3( 0, -1,  0),
            Self::Up    => glam::ivec3( 0,  1,  0),
            Self::North => glam::ivec3( 0,  0, -1),
            Self::South => glam::ivec3( 0,  0,  1),
            Self::West  => glam::ivec3(-1,  0,  0),
            Self::East  => glam::ivec3( 1,  0,  0),
        }
    }

    #[allow(clippy::match_same_arms)]
    #[must_use]
    #[rustfmt::skip]
    pub const fn to_cardinal_unit_vec(self) -> Option<glam::IVec2> {
        match self {
            Self::Down  => None,
            Self::Up    => None,
            Self::North => Some(glam::ivec2( 0, -1)),
            Self::South => Some(glam::ivec2( 0,  1)),
            Self::West  => Some(glam::ivec2(-1,  0)),
            Self::East  => Some(glam::ivec2( 1,  0)),
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub const fn from_unit_vec(vec: glam::IVec3) -> Option<Self> {
        match vec {
            glam::IVec3 { x:  0, y: -1, z:  0 } => Some(Self::Down),
            glam::IVec3 { x:  0, y:  1, z:  0 } => Some(Self::Up),
            glam::IVec3 { x:  0, y:  0, z: -1 } => Some(Self::North),
            glam::IVec3 { x:  0, y:  0, z:  1 } => Some(Self::South),
            glam::IVec3 { x: -1, y:  0, z:  0 } => Some(Self::West),
            glam::IVec3 { x:  1, y:  0, z:  0 } => Some(Self::East),
            _ => None,
        }
    }
}
