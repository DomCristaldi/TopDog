mod input_status_component;
mod character_movement_component;
mod player_avatar_component;

pub use self::input_status_component::InputStatusComponent;
pub use self::character_movement_component::CharacterMovementComponent;
pub use self::player_avatar_component::PlayerAvatarComponent;


mod simple;
pub use self::simple::*;
/*pub use self::simple::Dimensions as Simple::Dimensions;
pub use self::simple::Position as Simple::Position;
pub use self::simple::Velocity2D as Velocity2D;*/
