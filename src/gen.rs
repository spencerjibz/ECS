#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq)]

pub struct GenData {
     pub pos: usize,
     pub gen: u64,
}

pub struct EntityActive {
     active: bool,
     gen: u64,
}

// where we get new GenerationID

pub struct GenManager {
     items: Vec<EntityActive>,
     drops: Vec<usize>, // list of all dropped entities
}

impl GenManager {
     pub fn new() -> Self {

          GenManager {
               items: Vec::new(),
               drops: Vec::new(),
          }
     }

     pub fn next(&mut self) -> GenData {

          if let Some(loc) = self.drops.pop() {

               // most recent drop
               let ea = &mut self.items[loc];

               ea.active = true;

               ea.gen += 1;

               return GenData {
                    pos: loc,
                    gen: ea.gen,
               };
          }

          // if nothing left in the drops, add it to the end
          self.items.push(EntityActive {
               active: true,
               gen: 0,
          });

          GenData {
               gen: 0,
               pos: self.items.len() - 1,
          }
     }

     pub fn drop(&mut self, g: GenData) {

          if let Some(ea) = self.items.get_mut(g.pos) {

               if ea.active && ea.gen == g.gen {

                    // generation protection
                    // don't drop newer items than given
                    ea.active = false;

                    self.drops.push(g.pos);
               }
          }
     }
}
