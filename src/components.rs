use crate::textures::Texture;
use derive_deref::{Deref, DerefMut};
use shipyard::prelude::*;

pub struct Renderable {
    pub texture: Texture,
    pub flip: bool
}

pub struct BgLayer {
    pub layer: usize,
    pub left: EntityId
}

pub struct BgSprite {
    pub layer: usize
}

pub struct Sprite {}

#[derive(Deref, DerefMut)]
pub struct Velocity(pub nalgebra::Vector3<f64>);

#[derive(Deref)]
pub struct Hero(pub EntityId);