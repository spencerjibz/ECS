#![allow(dead_code, unused_imports, non_snake_case)]

mod data;
mod gen;
mod store;
mod system;

use data::*;
use gen::GenData;
use rand::prelude::*;
use std::time::{Duration, Instant};
use store::{EcsStore, VecStore};
use system::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{color, cursor, terminal_size};

use std::io::{stdout, Write};

fn main() {

     // get keyboard input in  a thready manner;
     let (ch_s, ch_r) = std::sync::mpsc::channel();

     std::thread::spawn(move || {

          let stdin = std::io::stdin();

          for k in stdin.keys() {

               // keys depends on TermRead trait
               ch_s.send(k).unwrap();
          }
     });

     let (w, h) = termion::terminal_size().unwrap(); // get terminal size
     let (w, _) = (w as i32, h as i32);

     let mut screen = std::io::stdout().into_raw_mode().unwrap();

     let mut gen = gen::GenManager::new();

     let mut strengths = store::VecStore::new();

     let mut dirs = store::VecStore::new();

     let mut pos = store::VecStore::new();

     let mut pass = 0;

     let now = Instant::now();

     createData(1);

     println!("{:?} ", now.elapsed());

     loop {

          //create one element per loop (choice not requirement)
          let g = gen.next();

          strengths.add(g, data::Strength { s: 1, h: 5 });

          dirs.add(g, data::Dir { vx: 0, vy: 0 });

          pos.add(
               g,
               data::Pos {
                    x: rand::random::<i32>() % w,
                    y: rand::random::<i32>() % w,
               },
          );

          dir_sys(&mut dirs, &mut pos);

          move_sys(&mut dirs, &mut pos);

          collision_sys(&pos, &mut strengths);

          death_sys(&mut gen, &mut strengths, &mut pos, &mut dirs);

          render_sys(&mut screen, &pos, &strengths);

          // print the number of passes that
          write!(&mut screen, "{} Pass ={}", cursor::Goto(1, 1), pass).ok();

          pass += 1;

          screen.flush().ok();

          while let Ok(Ok(k)) = ch_r.try_recv() {

               match k {
                    Key::Char('q') => return,
                    // Here handle any key presses to make the game do so things
                    _ => {}
               }
          }

          std::thread::sleep(Duration::from_millis(100));
     }
}

// create a function (createData) that will return VecStore<GenData>
// takes an input  T of u64
// from 0 to T, create GenData with GenData{ pos: random number, gen: i }
// use random number generator  for pos as usize
// use i as random number for gen
// return VecStore<GenData>
fn createData(t: u64) -> VecStore<GenData> {

     let mut store = VecStore::new();

     for i in 0..t {

          let pos = (i % t) as usize;

          let gen = i;

          store.add(GenData { pos, gen }, GenData { pos, gen });
     }

     store
}
