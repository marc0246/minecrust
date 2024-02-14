use std::borrow::Cow;
use std::fmt;
use std::hash::{BuildHasherDefault, Hash};
use std::marker::PhantomData;
use std::num::NonZeroU32;

use anyhow::{bail, Context, Result};
use bumpalo::Bump;
use enum_map::{Enum, EnumArray, EnumMap};
use rustc_hash::{FxHashMap, FxHasher};
use serde::de::value::MapAccessDeserializer;
use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::Deserialize;
use serde_derive_state::DeserializeState;
use serde_state::DeserializeState;
use strum::{EnumIter, IntoEnumIterator};
use world::resource::Location;
use world::{Block, BlockId, Direction};

use super::state;
use crate::{texture, Packs};

/// Stores the model of each block in the game.
#[derive(Debug)]
pub struct Registry {
    models: Box<EnumMap<BlockId, Model>>,
}

impl Registry {
    pub(crate) fn load(packs: Packs, texture_uploader: &mut dyn texture::Uploader) -> Result<Self> {
        // The original pack has close to 16 MiB of JSON files.
        let arena = Bump::with_capacity(16 * 1024 * 1024);
        let mut bakery = Bakery::new(packs, &arena, texture_uploader);
        let mut registry = Registry {
            models: Box::default(),
        };

        for block_id in BlockId::iter() {
            let block_name = block_id.to_location();
            let path = format!("assets/minecraft/blockstates/{block_name}.json");
            let (pack_index, mut file) = bakery
                .packs
                .find_file(0, &path)
                .with_context(|| format!("failed to load `{path}`"))?;
            let buffer = file.read_to_arena(bakery.arena)?;
            drop(file);

            let mut deserializer = serde_json::Deserializer::from_slice(buffer);

            bakery.block_id = block_id;
            bakery.pack_index = pack_index;
            let selectors = Selectors::deserialize_state(&mut bakery, &mut deserializer)
                .with_context(|| {
                    format!(
                        "failed to load `{}`",
                        bakery.packs.inner[pack_index].path.join(path).display(),
                    )
                })?;

            registry.models[block_id] = Model::new(selectors);
        }

        Ok(registry)
    }

    /// Returns the faces that should be rendered for a block and its state.
    #[must_use]
    pub fn faces_of(&self, block: Block) -> Cow<'_, Faces> {
        let model = &self.models[block.id()];

        match &model.selectors {
            Selectors::Variants(selectors) => {
                for selector in selectors {
                    if selector.when.call(block) {
                        return Cow::Borrowed(&selector.apply[0].faces); // FIXME
                    }
                }

                Cow::Owned(Faces::default())
            }
            Selectors::Multipart(selectors) => {
                let mut faces = Faces::default();

                for selector in selectors {
                    if selector.when.call(block) {
                        let fs = &selector.apply[0].faces; // FIXME

                        for direction in Direction::iter() {
                            faces.culled[direction].extend(fs.culled[direction].iter().copied());
                        }

                        faces.unculled.extend(fs.unculled.iter().copied());
                    }
                }

                Cow::Owned(faces)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// A freshly baked, grass fed, vegan block model. Ready for consumption.
#[derive(Debug)]
struct Model {
    ambient_occlusion: bool,
    gui_3d: bool,
    block_light: bool,
    item_transforms: ItemTransforms,
    selectors: Selectors,
}

impl Model {
    fn new(selectors: Selectors) -> Self {
        Model {
            ambient_occlusion: false,            // FIXME
            gui_3d: true,                        // FIXME
            block_light: true,                   // FIXME
            item_transforms: EnumMap::default(), // FIXME
            selectors,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            ambient_occlusion: true,
            gui_3d: true,
            block_light: true,
            item_transforms: ItemTransforms::default(),
            selectors: Selectors::Variants(Vec::default()),
        }
    }
}

type ItemTransforms = EnumMap<DisplayPlace, glam::Mat4>;

#[derive(Debug, DeserializeState)]
#[serde(
    bound = "'de: 'arena",
    de_parameters = "'arena",
    deserialize_state = "Bakery<'arena>"
)]
enum Selectors {
    #[serde(rename = "variants")]
    Variants(#[serde(deserialize_state_with = "deserialize_variants")] Vec<VariantSelector>),
    #[serde(rename = "multipart")]
    Multipart(#[serde(deserialize_state)] Vec<PartSelector>),
}

fn deserialize_variants<'de: 'arena, 'arena, D>(
    state: &mut Bakery<'arena>,
    deserializer: D,
) -> Result<Vec<VariantSelector>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(VariantsVisitor { state })
}

struct VariantsVisitor<'seed, 'arena> {
    state: &'seed mut Bakery<'arena>,
}

impl<'de: 'arena, 'arena> Visitor<'de> for VariantsVisitor<'_, 'arena> {
    type Value = Vec<VariantSelector>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut selectors = Vec::new();

        while let Some(when) =
            map.next_key_seed(state::VariantPredicateSeed { state: self.state })?
        {
            let apply = map.next_value_seed(WeightedEntriesSeed { state: self.state })?;
            selectors.push(VariantSelector { when, apply });
        }

        Ok(selectors)
    }
}

#[derive(Debug)]
struct VariantSelector {
    when: state::VariantPredicate,
    apply: Vec<WeightedEntry>,
}

#[derive(Debug, DeserializeState)]
#[serde(
    bound = "'de: 'arena",
    de_parameters = "'arena",
    deserialize_state = "Bakery<'arena>"
)]
struct PartSelector {
    #[serde(default, deserialize_state)]
    when: state::PartPredicate,
    #[serde(deserialize_state_with = "deserialize_weighted_entries")]
    apply: Vec<WeightedEntry>,
}

fn deserialize_weighted_entries<'de: 'arena, 'arena, D>(
    state: &mut Bakery<'arena>,
    deserializer: D,
) -> Result<Vec<WeightedEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(WeightedEntriesSeed { state })
}

struct WeightedEntriesSeed<'seed, 'arena> {
    state: &'seed mut Bakery<'arena>,
}

impl<'de: 'arena, 'arena> DeserializeSeed<'de> for WeightedEntriesSeed<'_, 'arena> {
    type Value = Vec<WeightedEntry>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de: 'arena, 'arena> Visitor<'de> for WeightedEntriesSeed<'_, 'arena> {
    type Value = Vec<WeightedEntry>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map or a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries = Vec::new();

        while let Some(entry) = seq.next_element_seed(WeightedEntrySeed { state: self.state })? {
            entries.push(entry);
        }

        if entries.is_empty() {
            Err(de::Error::invalid_length(0, &"a non-empty list"))
        } else {
            Ok(entries)
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let entry =
            WeightedEntrySeed { state: self.state }.deserialize(MapAccessDeserializer::new(map))?;

        Ok(vec![entry])
    }
}

#[derive(Debug)]
struct WeightedEntry {
    weight: u32,
    faces: Faces,
}

struct WeightedEntrySeed<'seed, 'arena> {
    state: &'seed mut Bakery<'arena>,
}

impl<'de: 'arena, 'arena> DeserializeSeed<'de> for WeightedEntrySeed<'_, 'arena> {
    type Value = WeightedEntry;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let props = ModelProperties::deserialize(deserializer)?;
        let weight = props.weight;
        let faces = self
            .state
            .load_model(&props)
            .map_err(|err| de::Error::custom(format_args!("{err:?}")))?;

        Ok(WeightedEntry { weight, faces })
    }
}

#[derive(Clone, Debug, Default)]
pub struct Faces {
    pub unculled: Vec<Quad>,
    pub culled: EnumMap<Direction, Vec<Quad>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub direction: Direction,
    pub vertices: [Vertex; 4],
    pub shade: bool,
    pub tint_index: i32,
}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub tex_index: u32,
    pub tex_coords: u32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Loads the different resources making up a block model and bakes them into a model that's ready
/// to be rendered.
pub(super) struct Bakery<'arena> {
    /// The resource packs that are being processed.
    packs: Packs,
    packs2: Packs,
    /// The block whose model is currently being baked.
    pub(super) block_id: BlockId,
    /// The pack which is currently being processed.
    pack_index: usize,
    /// Arena into which all JSON files are allocated.
    arena: &'arena Bump,
    /// Buffer for the currently-processed file from the pack.
    scratch: Vec<u8>,
    /// Keeps track of model files that have already been read and parsed.
    model_cache: FxHashMap<&'arena Location, &'arena RawModel<'arena>>,
    /// Stack used to resolve model dependencies.
    model_stack: FxIndexMap<&'arena Location, &'arena mut RawModel<'arena>>,
    /// Uploads textures as they are encountered. This actually outlives the arena, but since the
    /// lifetime is covariant, it doesn't make a difference.
    texture_uploader: &'arena mut dyn texture::Uploader,
    /// Keeps track of textures that have already been read, decoded and uploaded.
    texture_cache: FxHashMap<&'arena Location, texture::Info>,
    /// Stack used to resolve texture variable dependencies.
    texture_stack: FxIndexSet<TextureVariable<'arena>>,
    next_texture_index: u32,
}

impl<'arena> Bakery<'arena> {
    /// Creates a new `Bakery`.
    fn new(
        packs: Packs,
        arena: &'arena Bump,
        texture_uploader: &'arena mut dyn texture::Uploader,
    ) -> Self {
        let packs2 = packs.clone();

        Bakery {
            packs,
            packs2,
            arena,
            block_id: BlockId::Air,
            pack_index: 0,
            // The largest block file in the original pack is block/nether_portal.png at 14 KiB.
            scratch: vec![0; 64 * 1024],
            // The original pack has fewer than 2k models.
            model_cache: FxHashMap::with_capacity_and_hasher(2_000, BuildHasherDefault::default()),
            model_stack: FxIndexMap::with_capacity_and_hasher(10, BuildHasherDefault::default()),
            texture_uploader,
            // The original pack has fewer than 1k block textures.
            texture_cache: FxHashMap::with_capacity_and_hasher(
                1_000,
                BuildHasherDefault::default(),
            ),
            texture_stack: FxIndexSet::with_capacity_and_hasher(10, BuildHasherDefault::default()),
            next_texture_index: 0,
        }
    }

    /// Loads a model file from the current pack.
    fn load_model(&mut self, props: &ModelProperties<'arena>) -> Result<Faces> {
        let location = &props.model;
        let raw = self
            .load_raw_model(self.pack_index, location)
            .with_context(|| format!("failed to load model `{location}`"))?;
        let model_rotation =
            glam::Quat::from_rotation_x(props.x) * glam::Quat::from_rotation_y(props.y);
        let mut faces = Faces::default();

        for element in raw.elements() {
            for (direction, face) in element
                .faces
                .iter()
                .filter_map(|(k, v)| v.as_ref().map(|v| (k, v)))
            {
                let direction = direction.into();
                let mut texture_var = face.texture;

                let texture = loop {
                    match raw.find_texture_entry(texture_var) {
                        Some(TextureEntry::Location(location)) => {
                            break self
                                .load_texture(raw.pack_index, location)
                                .with_context(|| format!("failed to load texture `{location}`"))?;
                        }
                        Some(TextureEntry::Variable(other_var)) => {
                            self.texture_stack.insert(texture_var);

                            if self.texture_stack.contains(&other_var) {
                                bail!(
                                    "found dependency cycle while resolving texture variable: {}",
                                    self.texture_stack
                                        .iter()
                                        .chain(Some(&other_var))
                                        .map(|var| var.0)
                                        .intersperse(" -> ")
                                        .collect::<String>(),
                                );
                            }

                            texture_var = other_var;
                        }
                        None => {
                            bail!("found use of undefined texture variable `{texture_var}`");
                        }
                    }
                };

                self.texture_stack.clear();

                let quad = Quad {
                    direction,
                    vertices: make_vertices(direction, element, model_rotation, face.uvs, texture),
                    shade: element.shade,
                    tint_index: face.tint_index,
                };

                if let Some(cullface) = face.cullface {
                    let cullface = Direction::from_unit_vec(
                        (model_rotation * Direction::from(cullface).to_unit_vec().as_vec3())
                            .round()
                            .as_ivec3(),
                    )
                    .unwrap();
                    faces.culled[cullface].push(quad);
                } else {
                    faces.unculled.push(quad);
                }
            }
        }

        Ok(faces)
    }

    fn load_raw_model(
        &mut self,
        mut pack_index: usize,
        mut location: &'arena Location,
    ) -> Result<&'arena RawModel<'arena>> {
        // First we need to find the grandest parent in the dependency chain that is either already
        // cached or doesn't have a parent, because each cached model is immutable, so its parent
        // must be set before it can be put in the cache. For this we need to keep a dependency
        // stack for the models that have been encountered until such a model is reached (or a
        // dependency cycle is detected).
        let mut parent = loop {
            if let Some(&model) = self.model_cache.get(location) {
                // If the model is cached already then its parent(s) are cached and linked, by
                // virtue of this very algorithm.
                break model;
            }

            let (namespace, path) = location.as_parts();
            let path = format!("assets/{namespace}/models/{path}.json");
            let mut file;
            (pack_index, file) = self.packs.find_file(pack_index, &path)?;
            let model = self
                .arena
                .alloc(file.deserialize::<RawModel<'arena>>(self.arena)?);
            model.pack_index = pack_index;

            if let Some(parent_location) = model.parent_location {
                self.model_stack.insert(location, model);

                if self.model_stack.contains_key(&parent_location) {
                    bail!(
                        "found dependency cycle in model hierarchy: {}",
                        self.model_stack
                            .keys()
                            .chain(Some(&parent_location))
                            .map(|l| l.as_str())
                            .intersperse(" -> ")
                            .collect::<String>(),
                    );
                }

                location = parent_location;
            } else {
                self.model_cache.insert(location, model);
                break model;
            }
        };

        // Unwind the stack, linking each model with the preceding so that they can all be cached.
        for (location, model) in self.model_stack.drain(..).rev() {
            model.parent = Some(parent);
            self.model_cache.insert(location, model);
            parent = model;
        }

        // At this point, the parent has been unwound all the way back to the first model in the
        // dependency chain, aka the requested model.
        Ok(parent)
    }

    fn load_texture(
        &mut self,
        mut pack_index: usize,
        location: &'arena Location,
    ) -> Result<texture::Info> {
        if let Some(&texture_info) = self.texture_cache.get(location) {
            return Ok(texture_info);
        }

        let (namespace, location_path) = location.as_parts();
        let meta_path = format!("assets/{namespace}/textures/{location_path}.png.mcmeta");
        // Removes the `.mcmeta` suffix.
        let path = &meta_path[..meta_path.len() - 7];

        let file;
        (pack_index, file) = self.packs.find_file(pack_index, path)?;

        let mut reader = png::Decoder::new(file).read_info()?;

        let width = reader.info().width;
        let height = reader.info().height;

        let animation = if let Ok(mut file) = self.packs2.inner[pack_index].get(&meta_path) {
            let TextureMeta { animation } = file.deserialize(self.arena)?;

            Some(animation)
        } else {
            None
        };

        if let Some(animation) = &animation {
            // TODO:
            if height % width != 0 {
                bail!("found animated block texture with non-proportional dimensions");
            }

            let frame_count = height / width;

            for &i in &animation.frames {
                if i >= frame_count {
                    bail!("found invalid frame index `{i}`, there are only `{frame_count}` frames");
                }
            }
        }

        let bit_depth = reader.info().bit_depth as u8;

        if bit_depth == 16 {
            bail!("found unsupported bit depth `16`, only a bit depth of up to `8` is supported");
        }

        let index = self.next_texture_index;
        self.next_texture_index += 1;

        let texture_info = texture::Info {
            width,
            height,
            index,
        };

        self.texture_uploader
            .upload(texture_info, &mut |upload_buffer| {
                texture::read(&mut reader, upload_buffer, &mut self.scratch)
            })?;

        self.texture_cache.insert(location, texture_info);

        Ok(texture_info)
    }
}

type FxIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<FxHasher>>;

type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub(crate) fn resize_scratch(scratch: &mut Vec<u8>, new_len: usize) -> &mut [u8] {
    // We don't want to resize the scratchpad if it would truncate it, because then the previously
    // initialized bytes that are truncated would again have to be treated as uninitialized the
    // next time this function is called with a larger length, needlessly initializing them again.
    if new_len > scratch.len() {
        scratch.resize(new_len, 0);
    }

    &mut scratch[0..new_len]
}

// TODO: UV-locking
#[allow(clippy::too_many_lines)]
fn make_vertices(
    direction: Direction,
    element: &Element<'_>,
    model_rotation: glam::Quat,
    uvs: Option<[f32; 4]>,
    texture: texture::Info,
) -> [Vertex; 4] {
    use Direction::*;

    let vertices = |ps: [[f32; 3]; 4]| {
        let ps = ps.map(|p| {
            const MODEL_CENTER: glam::Vec3 = glam::vec3(0.5, 0.5, 0.5);

            let mut p = glam::Vec3::from(p);

            if let Some(rot) = &element.rotation {
                p -= rot.origin;
                p = rot.rotation * p;
                p *= rot.scale;
                p += rot.origin;
            }

            model_rotation * (p - MODEL_CENTER) + MODEL_CENTER
        });

        // Eliminates rounding errors.
        let ps = ps.map(|p| (p * 2f32.powi(21)).round() / 2f32.powi(21));

        let direction = Direction::from_unit_vec(
            (model_rotation * direction.to_unit_vec().as_vec3())
                .round()
                .as_ivec3(),
        )
        .unwrap();

        #[rustfmt::skip]
        let uvs = uvs.map_or_else(
            // If UVs are missing, set them equal to the vertex positions, modulo a dimension. In 
            // other words this is a simple projection from the 3D cuboid to a 2D plane. Some axes 
            // need to be inverted to turn the vertex coord into a texture coord.
            || match direction {
                Down  => [0.0 + ps[0].x, 1.0 - ps[0].z, 0.0 + ps[2].x, 1.0 - ps[2].z],
                Up    => [0.0 + ps[0].x, 0.0 + ps[0].z, 0.0 + ps[2].x, 0.0 + ps[2].z],
                North => [1.0 - ps[0].x, 0.0 + ps[0].y, 1.0 - ps[2].x, 0.0 + ps[2].y],
                South => [0.0 + ps[0].x, 0.0 + ps[0].y, 0.0 + ps[2].x, 0.0 + ps[2].y],
                West  => [0.0 + ps[0].z, 0.0 + ps[0].y, 0.0 + ps[2].z, 0.0 + ps[2].y],
                East  => [1.0 - ps[0].z, 0.0 + ps[0].y, 1.0 - ps[2].z, 0.0 + ps[2].y],
            },
            // Otherwise normalize the coords.
            |uvs| uvs.map(|c| c / 16.0),
        );

        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        let tex_coords = |u: f32, v: f32| {
            (((u * f32::from(u16::MAX)) as u32) << 16) | (v * f32::from(u16::MAX)) as u32
        };

        [
            Vertex {
                position: ps[0],
                tex_index: texture.index,
                tex_coords: tex_coords(uvs[0], uvs[1]),
            },
            Vertex {
                position: ps[1],
                tex_index: texture.index,
                tex_coords: tex_coords(uvs[0], uvs[3]),
            },
            Vertex {
                position: ps[2],
                tex_index: texture.index,
                tex_coords: tex_coords(uvs[2], uvs[3]),
            },
            Vertex {
                position: ps[3],
                tex_index: texture.index,
                tex_coords: tex_coords(uvs[2], uvs[1]),
            },
        ]
    };

    // Normalize coords.
    let (from, to) = (element.from / 16.0, element.to / 16.0);

    #[rustfmt::skip]
    match direction {
        Down => vertices([
            [from.x, from.y, to.z  ],
            [from.x, from.y, from.z],
            [to.x,   from.y, from.z],
            [to.x,   from.y, to.z  ],
        ]),
        Up => vertices([
            [from.x, to.y,   from.z],
            [from.x, to.y,   to.z  ],
            [to.x,   to.y,   to.z  ],
            [to.x,   to.y,   from.z],
        ]),
        North => vertices([
            [to.x,   to.y,   from.z],
            [to.x,   from.y, from.z],
            [from.x, from.y, from.z],
            [from.x, to.y,   from.z],
        ]),
        South => vertices([
            [from.x, to.y,   to.z  ],
            [from.x, from.y, to.z  ],
            [to.x,   from.y, to.z  ],
            [to.x,   to.y,   to.z  ],
        ]),
        West => vertices([
            [from.x, to.y,   from.z],
            [from.x, from.y, from.z],
            [from.x, from.y, to.z  ],
            [from.x, to.y,   to.z  ],
        ]),
        East => vertices([
            [to.x,   to.y,   to.z  ],
            [to.x,   from.y, to.z  ],
            [to.x,   from.y, from.z],
            [to.x,   to.y,   from.z],
        ]),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Describes a model.
#[derive(Debug, Deserialize)]
struct ModelProperties<'arena> {
    /// The associated model resource.
    #[serde(borrow)]
    model: &'arena Location,
    /// How much to rotate the model along the X-axis. Can only be in increments of 90 degrees.
    #[serde(default, deserialize_with = "deserialize_rotation")]
    x: f32,
    /// How much to rotate the model along the Y-axis. Can only be in increments of 90 degrees.
    #[serde(default, deserialize_with = "deserialize_rotation")]
    y: f32,
    /// Whether to lock the texture rotation, so that it doesn't rotate with the model.
    #[serde(default)]
    uvlock: bool,
    /// The probability that this model will be applied. This only matters if there are multiple to
    /// choose from.
    #[serde(default = "default_weight")]
    weight: u32,
}

fn deserialize_rotation<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let angle = u32::deserialize(deserializer)?;

    match angle {
        #[allow(clippy::cast_precision_loss)]
        0 | 90 | 180 | 270 => Ok((angle as f32).to_radians()),
        _ => Err(de::Error::custom(
            "rotation must be 0, 90, 180 or 270 degrees",
        )),
    }
}

fn default_weight() -> u32 {
    1
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// A block model file.
#[derive(Debug, Deserialize)]
#[serde(default)]
struct RawModel<'arena> {
    /// This is initialized by the model [`Bakery`].
    #[serde(skip)]
    parent: Option<&'arena Self>,
    /// This is initialized by the model [`Bakery`].
    #[serde(skip)]
    pack_index: usize,
    /// Unset fields should be inherited from this block model if present.
    #[serde(rename = "parent")]
    parent_location: Option<&'arena Location>,
    /// Whether to use ambient occlusion.
    #[serde(rename = "ambientocclusion")]
    ambient_occlusion: bool,
    /// Associates places where models are displayed with their respective transformations.
    #[serde(deserialize_with = "deserialize_enum_map")]
    display: EnumMap<DisplayPlace, Option<RawItemTransform>>,
    /// Associates texture variables the model uses with their respective resource locations of the
    /// textures, or with other texture variables.
    #[serde(borrow)]
    textures: FxHashMap<&'arena str, TextureEntry<'arena>>,
    /// The elements that make up the model.
    #[serde(borrow)]
    elements: Vec<Element<'arena>>,
}

impl<'arena> RawModel<'arena> {
    /// Gets the ambient occlusion setting of the root model in the hierarchy.
    fn ambient_occlusion(&self) -> bool {
        let mut model = self;

        while let Some(parent) = model.parent {
            model = parent;
        }

        model.ambient_occlusion
    }

    /// Gets the elements of the first model in the hierarchy that has them.
    fn elements(&self) -> &[Element<'arena>] {
        let mut model = self;

        loop {
            if !model.elements.is_empty() {
                break &model.elements;
            } else if let Some(parent) = model.parent {
                model = parent;
            } else {
                break &[];
            }
        }
    }

    /// Searches the model hierarchy for a texture entry.
    fn find_texture_entry(&self, var: TextureVariable<'arena>) -> Option<TextureEntry<'arena>> {
        let mut model = self;

        loop {
            if let Some(&texture) = model.textures.get(var.0) {
                break Some(texture);
            } else if let Some(parent) = model.parent {
                model = parent;
            } else {
                break None;
            }
        }
    }

    /// Finds all the item transforms individually and returns them.
    fn item_transforms(&self) -> ItemTransforms {
        let mut transforms = ItemTransforms::default();

        for place in DisplayPlace::iter() {
            transforms[place] = self.item_transform(place);
        }

        transforms
    }

    /// Finds the first model in the hierarchy that has a particular item transform and returns it.
    fn item_transform(&self, place: DisplayPlace) -> glam::Mat4 {
        let mut model = self;

        loop {
            if let Some(transform) = &model.display[place] {
                break transform.to_matrix();
            } else if let Some(parent) = &model.parent {
                model = parent;
            } else {
                break glam::Mat4::IDENTITY;
            }
        }
    }
}

impl Default for RawModel<'_> {
    fn default() -> Self {
        RawModel {
            parent: None,
            pack_index: usize::MAX,
            parent_location: None,
            ambient_occlusion: true,
            display: EnumMap::default(),
            textures: FxHashMap::default(),
            elements: Vec::new(),
        }
    }
}

fn deserialize_enum_map<'de, D, K, V>(deserializer: D) -> Result<EnumMap<K, Option<V>>, D::Error>
where
    D: Deserializer<'de>,
    K: EnumArray<Option<V>> + Deserialize<'de>,
    V: Deserialize<'de>,
{
    deserializer.deserialize_map(EnumMapVisitor {
        marker: PhantomData,
    })
}

struct EnumMapVisitor<K, V>
where
    K: EnumArray<Option<V>>,
{
    marker: PhantomData<fn() -> (K, V)>,
}

impl<'de, K, V> Visitor<'de> for EnumMapVisitor<K, V>
where
    K: EnumArray<Option<V>> + Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = EnumMap<K, Option<V>>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut enum_map = EnumMap::default();

        while let Some((key, value)) = map.next_entry()? {
            enum_map[key] = Some(value);
        }

        Ok(enum_map)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Places where items are displayed.
#[derive(Clone, Copy, Debug, Deserialize, Enum, EnumIter, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum DisplayPlace {
    /// Held in right hand and looked at in third person.
    ThirdpersonRighthand,
    /// Held in left hand and looked at in third person.
    ThirdpersonLefthand,
    /// Held in right hand and looked at in first person.
    FirstpersonRighthand,
    /// Held in left hand and looked at in first person.
    FirstpersonLefthand,
    /// In the inventory/in a container.
    Gui,
    /// Equipped in head slot.
    Head,
    /// Dropped block/item.
    Ground,
    /// Inside item frame.
    Fixed,
}

/// Item transformation.
#[derive(Debug, Deserialize)]
#[serde(default)]
struct RawItemTransform {
    rotation: [f32; 3],
    translation: [f32; 3],
    scale: [f32; 3],
}

impl RawItemTransform {
    fn to_matrix(&self) -> glam::Mat4 {
        let rotation = glam::Quat::from_euler(
            glam::EulerRot::XZY,
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
        );
        let translation = glam::Vec3::from(self.translation);
        let scale = glam::Vec3::from(self.scale);

        glam::Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }
}

impl Default for RawItemTransform {
    fn default() -> Self {
        RawItemTransform {
            rotation: [0.0; 3],
            translation: [0.0; 3],
            scale: [1.0; 3],
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Texture location or another variable which holds the location.
#[derive(Clone, Copy, Debug)]
enum TextureEntry<'arena> {
    Location(&'arena Location),
    Variable(TextureVariable<'arena>),
}

impl<'de: 'arena, 'arena> Deserialize<'de> for TextureEntry<'arena> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        if let Some(s) = s.strip_prefix('#') {
            if s.is_empty() {
                Err(de::Error::invalid_length(0, &"a non-empty variable name"))
            } else {
                Ok(TextureEntry::Variable(TextureVariable(s)))
            }
        } else {
            Location::new(s)
                .map(TextureEntry::Location)
                .map_err(de::Error::custom)
        }
    }
}

/// A texture variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TextureVariable<'arena>(&'arena str);

impl<'de: 'arena, 'arena> Deserialize<'de> for TextureVariable<'arena> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        let s = s.strip_prefix('#').ok_or_else(|| {
            de::Error::invalid_value(Unexpected::Str(s), &"a string starting with `#`")
        })?;

        if s.is_empty() {
            Err(de::Error::invalid_length(0, &"a non-empty variable name"))
        } else {
            Ok(TextureVariable(s))
        }
    }
}

impl fmt::Display for TextureVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// An element of a model. Can only be a cuboid.
#[derive(Debug, Deserialize)]
struct Element<'arena> {
    /// Start point of the cuboid.
    #[serde(deserialize_with = "deserialize_element_point")]
    from: glam::Vec3,
    /// End point of the cuboid.
    #[serde(deserialize_with = "deserialize_element_point")]
    to: glam::Vec3,
    /// Rotation of the cuboid.
    rotation: Option<ElementRotation>,
    /// Whether to render a shadow.
    #[serde(default = "default_shade")]
    shade: bool,
    /// The faces of the cuboid. A face shouldn't be rendered if it is not present.
    #[serde(borrow, default, deserialize_with = "deserialize_enum_map")]
    faces: EnumMap<FaceDirection, Option<ElementFace<'arena>>>,
}

fn deserialize_element_point<'de, D>(deserializer: D) -> Result<glam::Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    let p = deserialize_point(deserializer)?;

    if p.x < -16.0 || p.y < -16.0 || p.z < -16.0 || p.x > 32.0 || p.y > 32.0 || p.z > 32.0 {
        Err(de::Error::custom(
            "element points must be between -16.0 and 32.0",
        ))
    } else {
        Ok(p)
    }
}

fn deserialize_point<'de, D>(deserializer: D) -> Result<glam::Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    <[f32; 3]>::deserialize(deserializer).map(Into::into)
}

fn default_shade() -> bool {
    true
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct ElementRotation {
    origin: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

impl Default for ElementRotation {
    fn default() -> Self {
        ElementRotation {
            origin: glam::Vec3::ZERO,
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec3::ONE,
        }
    }
}

impl<'de> Deserialize<'de> for ElementRotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawElementRotation::deserialize(deserializer)?;
        let mut scale = glam::Vec3::splat(1.0 / raw.angle.cos());
        let mut axis = glam::Vec3::ZERO;

        match raw.axis {
            Axis::X => {
                scale.x = 1.0;
                axis.x = 1.0;
            }
            Axis::Y => {
                scale.y = 1.0;
                axis.y = 1.0;
            }
            Axis::Z => {
                scale.z = 1.0;
                axis.z = 1.0;
            }
        }

        Ok(ElementRotation {
            rotation: glam::Quat::from_axis_angle(axis, raw.angle),
            // Normalizes the coords.
            origin: raw.origin / 16.0,
            scale: if raw.rescale { scale } else { glam::Vec3::ONE },
        })
    }
}

/// Rotation of an element.
#[derive(Debug, Deserialize)]
struct RawElementRotation {
    /// Center of rotation.
    #[serde(deserialize_with = "deserialize_point")]
    origin: glam::Vec3,
    /// Axis of rotation.
    axis: Axis,
    /// Angle of rotation.
    #[serde(deserialize_with = "deserialize_angle")]
    angle: f32,
    /// Whether to scale the faces across the whole block.
    #[serde(default)]
    rescale: bool,
}

/// Axis of rotation.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Axis {
    X,
    Y,
    Z,
}

fn deserialize_angle<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    f32::deserialize(deserializer).and_then(|angle| {
        if [-45.0, -22.5, 0.0, 22.5, 45.0].contains(&angle) {
            Ok(angle.to_radians())
        } else {
            Err(de::Error::custom(
                "angle must be one of -45.0, -22.5, 0.0, 22.5 or 45.0 deg",
            ))
        }
    })
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Direction of a face.
#[derive(Clone, Copy, Debug, Deserialize, Enum, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum FaceDirection {
    // This alias is used in precisely one file in the original pack. Just for fun apparently.
    #[serde(alias = "bottom")]
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl From<FaceDirection> for Direction {
    fn from(face: FaceDirection) -> Self {
        #[rustfmt::skip]
        match face {
            FaceDirection::Down  => Direction::Down,
            FaceDirection::Up    => Direction::Up,
            FaceDirection::North => Direction::North,
            FaceDirection::South => Direction::South,
            FaceDirection::West  => Direction::West,
            FaceDirection::East  => Direction::East,
        }
    }
}

/// Defines the face of an element.
#[derive(Debug, Deserialize)]
struct ElementFace<'arena> {
    /// Area of the texture to map onto the face. Should be generated based on the element's
    /// position if unset.
    #[serde(rename = "uv")]
    uvs: Option<[f32; 4]>,
    /// The texture to use.
    #[serde(borrow)]
    texture: TextureVariable<'arena>,
    /// Prevents the face from being rendered when a block is next to it in the specified position.
    /// If unset then the position of the face should be used.
    cullface: Option<FaceDirection>,
    /// How much to rotate the texture by.
    #[serde(default, deserialize_with = "deserialize_rotation")]
    rotation: f32,
    ///
    #[serde(default = "default_tint_index", rename = "tintindex")]
    tint_index: i32,
}

fn default_tint_index() -> i32 {
    -1
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
struct TextureMeta {
    animation: Animation,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Animation {
    pub interpolate: bool,
    pub frametime: NonZeroU32,
    pub frames: Vec<u32>,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            interpolate: false,
            frametime: NonZeroU32::new(1).unwrap(),
            frames: Vec::new(),
        }
    }
}
