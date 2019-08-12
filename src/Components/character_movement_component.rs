extern crate amethyst;

use amethyst::{
	prelude::*,
	error::Error,
	core::transform::Transform,
	ecs::prelude::{
		Component,
		DenseVecStorage,
		WriteStorage 
		},
};

use crate::{
	Components::{
		Velocity2D,
	},
};

pub struct CharacterMovementComponent
{
	pub velocity: Velocity2D,
	pub momentum: f32,
}

impl Component for CharacterMovementComponent
{
	type Storage = DenseVecStorage<Self>;
}

impl CharacterMovementComponent
{
	pub fn default() -> CharacterMovementComponent
	{
		CharacterMovementComponent{
			velocity: Velocity2D::default(),
			momentum: 1.0,
		}
	}
}