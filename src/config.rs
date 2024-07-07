use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    /// Filename of the image with opaque processing only.
    /// Default: opaque.png
    #[serde(default = "default_opaque_image")]
    pub(crate) opaque_image: String,

    /// Filename of the processed for the X (formerly Twitter) header.
    /// Default: header.png
    #[serde(default = "default_header_image")]
    pub(crate) header_image: String,

    /// Type of sampling filter to use when resizing the image.
    /// Available filters: [Nearest, Triangle, CatmullRom, GAussian, Lanczos3]
    /// Default: CatmullRom
    #[serde(default = "default_filter")]
    pub(crate) filter: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            opaque_image: default_opaque_image(),
            header_image: default_header_image(),
            filter: default_filter(),
        }
    }
}

fn default_opaque_image() -> String {
    "opaque.png".to_string()
}

fn default_header_image() -> String {
    "header.png".to_string()
}

fn default_filter() -> String {
    // Set `CatmullRom` as default sampling filter because it has less noise and is not blurry.
    "CatmullRom".to_string()
}
