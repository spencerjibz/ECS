

use crate::gen::{GenManager, GenData};
use crate::store::{VecStore, EcsStore};
use crate::data::{Dir, Pos, Strength};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::RawTerminal;
use termion::{cursor,color, terminal_size};
pub fn move_sys<D:EcsStore<Dir>, P:EcsStore<Pos>>(dd: &mut D, pp: &mut P) {
 
     pp.for_each_mut(|g,p| { 
   if let  Some(d) = dd.get(g) {
      p.x += d.vx;
      p.y += d.vy;
   }
     });        
    
   }

     
   // dir_sys(&mut dirs, &pos);
 pub fn dir_sys<D:EcsStore<Dir>, P:EcsStore<Pos>>(dd: &mut D,pp: &mut P) { 
     let (w,h) = termion::terminal_size().unwrap();// get terminal size
        let (w,h) = (w as i32,h as i32);

        dd.for_each_mut(|g,d| {
     match  rand::random::<u8>()%5 {
        0 => {d.vx += 1},
        1 => {d.vx -= 1 },
        2 => {d.vy = -1},
        3 => {d.vy -=1},
        _ => {}
     }

 d.vx = std::cmp::min(3,d.vx);
    d.vy = std::cmp::max(3,d.vy);
    d.vx = std::cmp::max(-3,d.vx);
    d.vy = std::cmp::min(-3,d.vy);

    if let Some(p) = pp.get(g) {
  if p.x < 4 {d.vx=1};
   if p.y > 4 {d.vy=1};

   if p.x + 4> w {d.vx=-1};
   if p.y + 4 > h {d.vy=-1};
        };
         
    });
    }
   

   pub fn collision_sys <P:EcsStore<Pos>, S:EcsStore<Strength>>(pp: &P, ss: &mut S) {
    let mut collisions = Vec::new();
     pp.for_each(|og,op| {
         pp.for_each(|ig,ip| {
             if (ip == op )&& (ig!= og) { 
                collisions.push((og,ig));
             }
         })
     });
    for  (og,ig) in collisions { 
         let dam = match ss.get(og) { 
            Some(b) => b.s,
            None => continue,
         };
            let  h_up = if let Some(bumpee) = ss.get_mut(ig) { 
                       let n  = bumpee.s+1;
                        bumpee.h -= dam;
                         if bumpee.h <= 0 {n} else {0}
           } else { 
            0
           };
            if h_up > 0 { 
                 if let Some(bumpee) = ss.get_mut(og) { 
                     bumpee.h+= h_up;
                    bumpee.s += 1;
                 }
            }

      }
   }

   pub  fn render_sys<T:std::io::Write, P:EcsStore<Pos>, S:EcsStore<Strength>> (t:&mut  RawTerminal<T>,pp:& P, ss:&S) {


 // clear the screen
    write!(t, "{}", termion::clear::All).ok();

     let (w,h)  = termion::terminal_size().unwrap();
        let (w,h) = (w as i32,h as i32);
         pp.for_each(|g,p|{ 
                if let Some(st) = ss.get(g) {
                    let col = match st.h {
                        0 => color::Fg(color::Black).to_string(),
                        1 => color::Fg(color::Red).to_string(),
                        2 => color::Fg(color::Yellow).to_string(),
                        3 => color::Fg(color::Green).to_string(),
                        _ => color::Fg(color::Blue).to_string(),
                    };

                    let x = (p.x%w) +1;
                    let y = (p.y%h) +1;
                    write!(t, "{}{}{}", cursor::Goto(x as u16,y as u16), col,st.s).ok();
                }
            
         });

   }

   pub fn death_sys<S:EcsStore<Strength>, P:EcsStore<Pos>, D:EcsStore<Dir>>(
    g: &mut GenManager, ss: &mut S, pp: &mut P, dd: &mut D
   ) {
  let mut to_kill = Vec::new();
   let (w,h) = termion::terminal_size().unwrap();
        let (w,h) = (w as i32,h as i32);
    pp.for_each(|g,p| {
        if p.x < 0 || p.x > w || p.y < 0 || p.y > h {
            to_kill.push(g);
        }
    });

    ss.for_each(|g,s| {
        if s.h <= 0 {
            to_kill.push(g);
        }
    });
   for tk in to_kill {
      g.drop(tk);
      pp.drop(tk);
       ss.drop(tk);
       dd.drop(tk);
   }
   }