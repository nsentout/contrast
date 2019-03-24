use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::color::Color;
use mark_macro_derive::MarkMacro;

use std::collections::HashMap;
use std::collections::LinkedList;

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
    pub fn new(name: String, bitmap: Vec<f32>, rect: Rect, adv: i64, bx: i32, by: i32) -> Glyph
    {
        Glyph{name, bitmap, rect, adv: adv as i32, bx, by}
    }

    pub fn empty() -> Glyph
    {
        Glyph{name: String::from(""), bitmap: Vec::new(), rect: Rect::new(0, 0, 0, 0), adv: 0, bx: 0, by: 0}
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
        face.set_pixel_sizes(0, police).unwrap();

        self.cached.insert(name.to_string(), FaceCache::new(face, name.to_string()));
    }

    pub fn get_face(&mut self, name: &str) -> Option<&mut FaceCache>
    {
        self.cached.get_mut(name)
    }

    pub fn contains(&self, name: &str) -> bool
    {
        self.cached.contains_key(name)
    }
}

impl FaceCache
{
    pub fn new(face: freetype::Face, name: String) -> FaceCache
    {
        let mut cache = FaceCache{face, name, chars: HashMap::new(), atlas: Packer::new(*ATLAS), writable: LinkedList::new()};
        cache.prepare_string(ASCII);
        cache
    }

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

                let g =  Glyph::new(self.name.clone(), bitmap, rect, glyph.advance().x, glyph.bitmap_left(), glyph.bitmap_top());
                if c == '!' { self.chars.insert(' ', g.clone()); }
                self.chars.insert(c, g.clone());
                self.writable.push_front(g);
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

            x += glyph.adv >> 6;
            if c == ' ' { continue; }

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
        }

        vertices
    }

    pub fn get_writable(&mut self) -> LinkedList<Glyph>
    {
        let mut list = LinkedList::new();
        list.append(&mut self.writable);
        list
    }
}

pub struct TextMarkCmd
{
    pub name: String,
    pub color: Color,
    pub start: usize,
    pub end: usize
}

impl TextMarkCmd
{
    pub fn new(name: &str, color: Color, start: usize, end: usize) -> TextMarkCmd
    {
        TextMarkCmd{name: name.to_string(), color, start, end}
    }
}

#[derive(MarkMacro, Clone)]
pub struct TextMark
{
    pub(crate) common_properties: MarkProperties,
    pub(crate) face: String,
    pub(crate) text: String,
    pub(crate) x: i32,
    pub(crate) y: i32
}

impl TextMark
{
    pub fn new() -> TextMark
    {
        TextMark{common_properties: MarkProperties::new(), face: String::from(""), text: String::from(""), x: 0, y: 0}
    }

    pub fn set_font(&mut self, face: &str) -> &mut Self
    {
        self.face = face.to_string();
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self
    {
        self.text = text.to_string();
        self
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> &mut Self
    {
        self.x = x;
        self.y = y;
        self
    }

    pub fn get_font(&self) -> &String
    {
        &self.face
    }
    
    pub fn get_text(&self) -> &String
    {
        &self.text
    }

    pub fn get_x(&self) -> i32 { self.x }

    pub fn get_y(&self) -> i32 { self.y }
}