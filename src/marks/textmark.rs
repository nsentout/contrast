use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::position::Position;
use mark_macro_derive::MarkMacro;

use std::collections::HashMap;

use rect_packer::{Packer, Rect};

const SIZE: &'static f32 = &1024.0;
const ATLAS: &'static rect_packer::Config = &rect_packer::Config{width: 1024, height: 1024, border_padding: 5, rectangle_padding: 10};
const ASCII: &'static str = &"!\"#$%&\'()*+,-./:;<=>?[]\\|{}^~_@`abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub type VertexText = ([f32; 4]);

pub struct FontCache
{
    pub(crate) library: freetype::Library,
    pub(crate) cached: HashMap<String,FaceCache>
}

pub struct Glyph
{
    bitmap: Vec<f32>,
    rect: Rect,
    adv: i32,
    bx: i32,
    by: i32
}

pub struct FaceCache
{
    pub(crate) face: freetype::Face,
    pub(crate) chars: HashMap<char,Glyph>,
    pub(crate) atlas: Packer
}

impl Glyph
{
    pub fn new(bitmap: Vec<f32>, rect: Rect, adv: i64, bx: i32, by: i32) -> Glyph
    {
        Glyph{bitmap, rect, adv: adv as i32, bx, by}
    }

    pub fn empty() -> Glyph
    {
        Glyph{bitmap: Vec::new(), rect: Rect::new(0, 0, 0, 0), adv: 0, bx: 0, by: 0}
    }
}

impl FontCache
{
    pub fn new() -> FontCache
    {
        FontCache{library: freetype::Library::init().unwrap(), cached: HashMap::new()}
    }

    pub fn create_face(&mut self, name: &str, font: &str, police: u32)
    {
        let face = self.library.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, police);

        self.cached.insert(name.to_string(), FaceCache::new(face));
    }

    pub fn get_face(&self, name: &str) -> Option<&FaceCache>
    {
        self.cached.get(name)
    }
}

impl FaceCache
{
    pub fn new(face: freetype::Face) -> FaceCache
    {
        let mut cache = FaceCache{face, chars: HashMap::new(), atlas: Packer::new(*ATLAS)};
        cache.prepare_string(ASCII);
        cache
    }

    pub fn prepare_string(&mut self, s: &str)
    {
        for c in s.chars()
        {
            self.face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
            let glyph = self.face.glyph();

            if let Some(rect) = self.atlas.pack(glyph.bitmap().width(), glyph.bitmap().rows(), false)
            {
                let n = rect.width as usize;
                let mut bitmap = Vec::with_capacity(n.pow(2));
                let data = glyph.bitmap().buffer().iter().map(|&x| { x as f32}).collect::<Vec<_>>();
                let mut chunks = data.chunks(n).collect::<Vec<_>>();
                chunks.reverse();
                for v in chunks { bitmap.extend_from_slice(v); }

                self.chars.insert(c, Glyph::new(bitmap, rect, glyph.advance().x, glyph.bitmap_left(), glyph.bitmap_top()));
            }
        }
    }

    pub fn drawing_commands(&self, x: i32, y: i32, text: &str) -> Vec<VertexText>
    {
        let mut x = x;
        let mut vertices = Vec::new();

        for c in text.chars()
        {
            let glyph = self.chars.get(&c).unwrap();

            let xpos = (x + glyph.bx) as f32;
            let ypos = (y - glyph.by) as f32;

            let w: f32 = glyph.rect.width as f32;
            let h: f32 = glyph.rect.height as f32;

            let u: f32 = (glyph.rect.x as f32) / SIZE;
            let v: f32 = (glyph.rect.y as f32) / SIZE;
            let u2: f32 = ((glyph.rect.x as f32) + w) / SIZE;
            let v2: f32 = ((glyph.rect.y as f32) + h) / SIZE;

            vertices.push([xpos  , ypos+h, u , v ]);
            vertices.push([xpos  , ypos  , u , v2]);
            vertices.push([xpos+w, ypos  , u2, v2]);

            vertices.push([xpos  , ypos+h, u , v ]);
            vertices.push([xpos+w, ypos  , u2, v2]);
            vertices.push([xpos+w, ypos+h, u2, v ]);

            x += glyph.adv >> 6;
        }

        vertices
    }
}