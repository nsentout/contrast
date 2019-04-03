use crate::MarkMacro;
use properties::position::Position;
use properties::color::Color;
use properties::markid::MarkId;
use mark_macro_derive::MarkMacro;

use std::collections::HashMap;
use std::collections::LinkedList;

use rect_packer::{Packer, Rect};

/// Texture atlas size.
const SIZE: &'static f32 = &1024.0;
/// Static atlas config.
const ATLAS: &'static rect_packer::Config = &rect_packer::Config{width: 1024, height: 1024, border_padding: 5, rectangle_padding: 10};
/// Default ascii chars.
const ASCII: &'static str = &"!\"#$%&\'()*+,-./:;<=>?[]\\|{}^~_@`abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// This is the type that will receive our shaders when we will want to render our text marks.
/// We could describe it this way to be clearer :
/// type VertexText = (position, texture coordinates).
pub type VertexText = ([f32; 3],[f32; 2]);

/// Cache Font & Freetype library.
pub struct FontCache
{
    pub(crate) library: freetype::Library,
    pub(crate) cached: HashMap<String,FaceCache>
}

/// Stores glyph metrics, bitmap & face cache name.
#[derive(Clone)]
pub struct Glyph
{
    pub name: String,
    pub bitmap: Vec<f32>,
    pub rect: Rect,
    adv: i32,
    bx: i32,
    by: i32
}

/// Stores a font face & all her loaded glyphs.
/// A face is a font loaded with a police size.
/// Generally contrast calls a font name, the key that stores a FaceCache.
#[derive(Clone)]
pub struct FaceCache
{
    pub(crate) face: freetype::Face,
    pub(crate) name: String,
    pub(crate) chars: HashMap<char,Glyph>,
    pub(crate) atlas: Packer,
    pub(crate) writable: LinkedList<Glyph>
}

impl Glyph
{
    /// Creates a new glyph.
    pub fn new(name: String, bitmap: Vec<f32>, rect: Rect, adv: i64, bx: i32, by: i32) -> Glyph
    {
        Glyph{name, bitmap, rect, adv: adv as i32, bx, by}
    }

    /// Creates an empty glyph.
    pub fn empty() -> Glyph
    {
        Glyph{name: String::from(""), bitmap: Vec::new(), rect: Rect::new(0, 0, 0, 0), adv: 0, bx: 0, by: 0}
    }
}

impl FontCache
{
    /// Creates & initialize Freetype.
    pub fn new() -> FontCache
    {
        FontCache{library: freetype::Library::init().unwrap(), cached: HashMap::new()}
    }

    /// Create a FaceCache and store it with the associated font name.
    /// Be careful, the font name is unique and does not match the actual name of the font, it just serves to store a FaceCache.
    pub fn create_face(&mut self, name: &str, font: &str, police: u32)
    {
        let face = self.library.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, police).unwrap();

        self.cached.insert(name.to_string(), FaceCache::new(face, name.to_string()));
    }

    /// Returns a FaceCache, if it exists.
    pub fn get_face(&mut self, name: &str) -> Option<&mut FaceCache>
    {
        self.cached.get_mut(name)
    }

    /// Checks if a font is registered.
    pub fn contains(&self, name: &str) -> bool
    {
        self.cached.contains_key(name)
    }
}

impl FaceCache
{
    /// Stores a new face & loads her glyphs.
    pub fn new(face: freetype::Face, name: String) -> FaceCache
    {
        let mut cache = FaceCache{face, name, chars: HashMap::new(), atlas: Packer::new(*ATLAS), writable: LinkedList::new()};
        // Default load ascii characters.
        cache.prepare_string(ASCII);
        cache
    }

    /// Loads char glyphs contained in the string.
    pub fn prepare_string(&mut self, s: &str)
    {
        for c in s.chars()
        {
            if self.chars.contains_key(&c) { continue; }
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

                let g =  Glyph::new(self.name.clone(), bitmap, rect, glyph.advance().x.into(), glyph.bitmap_left(), glyph.bitmap_top());
                if c == '!' { self.chars.insert(' ', g.clone()); }
                self.chars.insert(c, g.clone());
                self.writable.push_front(g);
            }
        }
    }

    /// Builds vertices in accordance with a position & a content.
    pub fn drawing_commands(&self, x: i32, y: i32, z: f32, text: &str) -> Vec<VertexText>
    {
        let mut x = x;
        let mut vertices = Vec::new();

        for c in text.chars()
        {
            let glyph = self.chars.get(&c).unwrap();

            let xpos = (x + glyph.bx) as f32;
            let ypos = (y - glyph.by) as f32;

            x += glyph.adv >> 6;
            if c == ' ' { continue; }

            let w: f32 = glyph.rect.width as f32;
            let h: f32 = glyph.rect.height as f32;

            let u: f32 = (glyph.rect.x as f32) / SIZE;
            let v: f32 = (glyph.rect.y as f32) / SIZE;
            let u2: f32 = ((glyph.rect.x as f32) + w) / SIZE;
            let v2: f32 = ((glyph.rect.y as f32) + h) / SIZE;

            vertices.push(([xpos  , ypos+h, z], [u , v ]));
            vertices.push(([xpos  , ypos  , z], [u , v2]));
            vertices.push(([xpos+w, ypos  , z], [u2, v2]));

            vertices.push(([xpos  , ypos+h, z], [u , v ]));
            vertices.push(([xpos+w, ypos  , z], [u2, v2]));
            vertices.push(([xpos+w, ypos+h, z], [u2, v ]));
        }

        vertices
    }

    /// Returns news loaded glyphs that need to be updated on a texture.
    pub fn get_writable(&mut self) -> LinkedList<Glyph>
    {
        let mut list = LinkedList::new();
        list.append(&mut self.writable);
        list
    }
}

/// Command needed to draw a mark text correctly.
/// Store :
/// - The font name (key to FaceCache & Texture)
/// - The color
/// - Indexes in vertices
pub struct TextMarkCmd
{
    pub name: String,
    pub color: Color,
    pub start: usize,
    pub end: usize
}

impl TextMarkCmd
{
    /// Creates a new Cmd.
    pub fn new(name: &str, color: Color, start: usize, end: usize) -> TextMarkCmd
    {
        TextMarkCmd{name: name.to_string(), color, start, end}
    }
}

/// This is the structure that describes the marks of type Text.
/// Structure directly manipulable by the user.
/// The font is only a string to avoid copy.
/// The mark only contains the key to the FontCache.
#[derive(PartialEq, Debug, MarkMacro, Clone)]
pub struct TextMark
{
    pub(crate) markid : MarkId,
    pub(crate) color : Color,
    pub(crate) face: String,
    pub(crate) text: String,
    pub(crate) pos: Position
}

impl TextMark
{
    /// Returns a new empty instance of TextMark.
    pub fn new() -> TextMark
    {
        TextMark{markid: MarkId::new(), color : Color::default(), face: String::from(""), text: String::from("")
                , pos: Position{x: 0.0, y: 0.0, z:0.0}}
    }

    /// Setter of the font.
    pub fn set_font(&mut self, face: &str) -> &mut Self
    {
        self.face = face.to_string();
        self
    }

    /// Setter of the text content.
    pub fn set_text(&mut self, text: &str) -> &mut Self
    {
        self.text = text.to_string();
        self
    }

    /// Setter of the position
    pub fn set_position<P : Into <Position>>(&mut self, position: P) -> &mut Self
    {
        self.pos = position.into();
        self
    }

    /// Borrow the position.
    pub fn get_position(&self) -> &Position
    {
        &self.pos
    }

    /// Borrow the font name.
    pub fn get_font(&self) -> &String
    {
        &self.face
    }
    
    /// Borrow the text content.
    pub fn get_text(&self) -> &String
    {
        &self.text
    }

    /// Get the X-coord in i32.
    pub fn get_x(&self) -> i32 { self.pos.x as i32 }

    /// Get the Y-coord in i32.
    pub fn get_y(&self) -> i32 { self.pos.y as i32 }

    /// Get the Z-coord in f32.
    pub fn get_z(&self) -> f32 { self.pos.z }
}