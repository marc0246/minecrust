//! Parsing of resource packs.

#![feature(
    iter_intersperse,
    inline_const,
    slice_as_chunks,
    slice_flatten,
    stmt_expr_attributes
)]

use std::alloc::Layout;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io, slice};

use anyhow::{bail, Context, Result};
use bumpalo::Bump;
use serde::Deserialize;
use sync_file::SyncFile;
use zip::read::ZipFile;
use zip::ZipArchive;

use self::block::{model, texture};

pub mod block;

/// Loads a set of resource packs.
pub fn load_packs(
    packs: Vec<Pack>,
    texture_uploader: &mut dyn texture::Uploader,
) -> Result<model::Registry> {
    model::Registry::load(packs.into(), texture_uploader)
}

/// A list of resource packs, where packs of lower indices override packs of higher indices.
#[derive(Debug, Clone)]
struct Packs {
    inner: Vec<Pack>,
}

impl Packs {
    /// Finds the first pack in the list that contains the given path, starting at the given index.
    /// Returns the index of the pack that contained the file as well as the file itself.
    fn find_file(&mut self, starting_index: usize, path: &str) -> Result<(usize, File<'_>)> {
        for (index, pack) in self.inner.iter_mut().enumerate().skip(starting_index) {
            if let Ok(file) = pack.get(path) {
                return Ok((index, file));
            }
        }

        bail!("specified file not found in the hierarchy of selected resource packs");
    }
}

impl From<Vec<Pack>> for Packs {
    fn from(inner: Vec<Pack>) -> Self {
        Packs { inner }
    }
}

/// Represents a resource pack, so either a zipped archive or unzipped directory.
#[derive(Debug, Clone)]
pub struct Pack {
    path: PathBuf,
    archive: Option<ZipArchive<SyncFile>>,
}

impl Pack {
    /// Creates a new `Pack` from the given path by automatically detecting whether it's zipped.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let archive = if path.is_dir() {
            None
        } else {
            Some(ZipArchive::new(SyncFile::open(path)?)?)
        };

        Ok(Pack {
            path: path.to_owned(),
            archive,
        })
    }

    /// Retrieves a file from the pack.
    fn get(&mut self, path: &str) -> Result<File<'_>> {
        let context = || format!("failed to open `{}`", self.path.join(path).display());

        if let Some(archive) = &mut self.archive {
            archive
                .by_name(path)
                .map(|file| File::Zipped {
                    file,
                    pack_path: &self.path,
                })
                .with_context(context)
        } else {
            let path = self.path.join(path);

            fs::File::open(&path)
                .map(|file| File::Unzipped { file, path })
                .with_context(context)
        }
    }
}

/// A file from a resource [`Pack`].
#[allow(clippy::large_enum_variant)] // Zipped files are far more common than unzipped files.
enum File<'pack> {
    Unzipped {
        file: fs::File,
        path: PathBuf,
    },
    Zipped {
        file: ZipFile<'pack>,
        pack_path: &'pack Path,
    },
}

impl File<'_> {
    fn read_to_arena<'arena>(&mut self, arena: &'arena Bump) -> Result<&'arena [u8]> {
        let size = self.size()?.try_into().unwrap();
        let buffer_ptr = arena.alloc_layout(Layout::from_size_align(size, 1).unwrap());

        // SAFETY: Hopes and prayers. This is *currently* sound because the reference is only used
        // for writing. Once `read_buf` is stabilized and/or implemented by `zip` we can use that.
        let buffer = unsafe { slice::from_raw_parts_mut(buffer_ptr.as_ptr(), size) };

        self.read_exact(buffer)
            .with_context(|| format!("failed to read `{}`", self.path().display()))?;

        Ok(buffer)
    }

    fn deserialize<'arena, T>(&mut self, arena: &'arena Bump) -> Result<T>
    where
        T: Deserialize<'arena>,
    {
        let buffer = self.read_to_arena(arena)?;

        serde_json::from_slice(buffer)
            .with_context(|| format!("failed to parse `{}`", self.path().display()))
    }

    fn size(&self) -> Result<u64> {
        let size = match self {
            File::Unzipped { file, .. } => file.metadata()?.len(),
            File::Zipped { file, .. } => file.size(),
        };

        Ok(size)
    }

    fn path(&self) -> PathBuf {
        match self {
            File::Unzipped { path, .. } => path.clone(),
            File::Zipped { file, pack_path } => pack_path.join(file.name()),
        }
    }
}

impl io::Read for File<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            File::Unzipped { file, .. } => file.read(buf),
            File::Zipped { file, .. } => file.read(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match self {
            File::Unzipped { file, .. } => file.read_exact(buf),
            File::Zipped { file, .. } => file.read_exact(buf),
        }
    }
}
