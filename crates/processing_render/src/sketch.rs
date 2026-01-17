//! A Sketch asset represents a source file containing user code for a Processing sketch.
//!
//! Sketches are loaded through Bevy's asset system, which provides automatic file watching
//! and change detection. This enables hot-reloading workflows where artists can edit their
//! sketch code and see changes reflected immediately without restarting.
//!
//! This module is intentionally language-agnostic — it only handles loading source text from
//! disk. Language-specific crates (like `processing_pyo3`) are responsible for executing the
//! source and binding it to the Processing API.

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use std::path::PathBuf;

/// Plugin that registers the Sketch asset type and its loader.
pub struct LivecodePlugin;

impl Plugin for LivecodePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Sketch>()
            .init_asset_loader::<SketchLoader>();
    }
}

/// A sketch source file loaded as a Bevy asset.
///
/// The `Sketch` asset contains the raw source code as a string. It does not interpret
/// or execute the code — that responsibility belongs to language-specific crates.
#[derive(Asset, TypePath, Debug)]
pub struct Sketch {
    /// The source code contents of the sketch file.
    pub source: String,

    /// The original file path.
    pub path: PathBuf,
}

/// Loads sketch files from disk.
///
/// Currently supports `.py` files, but the loader is designed to be extended
/// for other languages in the future.
#[derive(Default)]
pub struct SketchLoader;

impl AssetLoader for SketchLoader {
    type Asset = Sketch;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut source = String::new();

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        if let Ok(utf8) = str::from_utf8(&bytes) {
            source = utf8.to_string();
        }

        let asset_path = load_context.path();
        let path: PathBuf = asset_path.path().to_path_buf();

        Ok(Sketch { source, path })
    }

    fn extensions(&self) -> &[&str] {
        &["py"]
    }
}
