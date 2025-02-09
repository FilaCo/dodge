use crate::ecs::{Entity, System, World};
use crate::lexing::{LexingRequested, TokenRequested};

#[derive(Debug)]
pub struct LexingSystem;

impl System for LexingSystem {
    fn update(&self, w: &World) {}
}
