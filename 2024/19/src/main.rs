use std::marker::PhantomData;
// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<State> {
  _state: PhantomData<State>
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
  pub fn new() -> Self {
    Self { _state: PhantomData }
  }
  pub fn load(self) -> Sleigh<Ready>{
    Sleigh { _state: PhantomData }
  }
}

impl Sleigh<Ready> {
  pub fn take_off(self) -> Sleigh<Flying> {
    Sleigh { _state: PhantomData }
  }
  pub fn unload(self) ->  Sleigh<Empty> {
    Sleigh { _state: PhantomData }
  }
}

impl Sleigh<Flying> {
  pub fn land(self) -> Sleigh<Ready> {
    Sleigh { _state: PhantomData }
  }
}

pub fn main() {
  let sleigh = Sleigh::new();
  let loaded_sleigh = sleigh.load();
  let flying_sleigh = loaded_sleigh.take_off();
  let ready_sleigh = flying_sleigh.land();
  let empty_sleigh = ready_sleigh.unload();
  println!("{:?}", empty_sleigh._state);
}
