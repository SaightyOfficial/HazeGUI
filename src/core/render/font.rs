use std::collections::HashMap;
use std::sync::Mutex;
use fontdue::{Font, FontSettings, Metrics};

static FONT_DATA: &[u8] = include_bytes!("../../../fonts/JetBrainsMono-Regular.ttf");

//Glyph cache and font data
lazy_static::lazy_static! {
    pub static ref GLYPH_CACHE: Mutex<HashMap<(char, u32), (Metrics, Vec<u8>)>> = Mutex::new(HashMap::new());
    pub static ref FONT: Font = {
        Font::from_bytes(FONT_DATA, FontSettings::default()).expect("Font load error")
    };
}

///Rasterizes character and saves it to glyph cache
pub fn get_glyph(c: char, font_size: f32) -> (Metrics, Vec<u8>) {
    let size_key = (font_size * 100.0) as u32;
    let mut cache = GLYPH_CACHE.lock().expect("Poisoned glyph cache");

    if let Some(glyph) = cache.get(&(c, size_key)) {
        return glyph.clone();
    }

    //Rasterising font and saving it
    let (metrics, bitmap) = FONT.rasterize(c, font_size);
    cache.insert((c, size_key), (metrics.clone(), bitmap.clone()));
    
    (metrics, bitmap)
}