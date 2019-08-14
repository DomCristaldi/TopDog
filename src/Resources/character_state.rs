

/*pub struct CharacterState;

impl State*/

pub struct CharacterStateData;

pub trait CharacterState
{
    /// Executed when the game state begins.
    fn on_start(&mut self, _data: StateData<'_, ()>) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self, _data: StateData<'_, ()>) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _data: StateData<'_, ()>) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _data: StateData<'_, ()>) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_event(&mut self, _data: StateData<'_, ()>, event: StateEvent) -> EmptyTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(&mut self, _data: StateData<'_, ()>) -> EmptyTrans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit).
    fn update(&mut self, _data: StateData<'_, ()>) -> EmptyTrans {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default),
    /// even when this is not the active state,
    /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
    fn shadow_fixed_update(&mut self, _data: StateData<'_, ()>) {}

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit),
    /// even when this is not the active state,
    /// as long as this state is on the [StateMachine](struct.StateMachine.html)'s state-stack.
    fn shadow_update(&mut self, _data: StateData<'_, ()>) {}
}