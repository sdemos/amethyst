use amethyst_assets::Handle;
use amethyst_core::ecs::{Component, DenseVecStorage};
use amethyst_rendy::sprite::TextureCoordinates;
use amethyst_rendy::{SpriteRender, Texture};

/// Image used UI widgets, often as background.
#[derive(Debug, Clone, PartialEq)]
pub enum UiImage {
    /// An image backed by texture handle
    Texture(Handle<Texture>),
    /// An image backed by partial texture cropped
    Partial_Texture(Handle<Texture>, TextureCoordinates),
    /// An image backed by a Sprite
    Sprite(SpriteRender),
    /// An image entirely covered by single solid color
    SolidColor([f32; 4]),
}

impl Component for UiImage {
    type Storage = DenseVecStorage<Self>;
}
