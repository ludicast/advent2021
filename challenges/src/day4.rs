use std::collections::HashSet;

use file_reader::{get_bingos};

pub fn day4_new() {
    let bingos = get_bingos("data/bingos.txt");

    let len = bingos.boards.len();

    let ball_sack: &mut HashSet<i64> = &mut HashSet::new();
    let old_wins: &mut HashSet<usize> = &mut HashSet::new();
    
    for ball in bingos.balls {
      ball_sack.insert(ball);
      for i in 0..len {
        if old_wins.contains(&i) {
          continue;
        }
        let board = &bingos.boards[i]; 
        if board.has_bingo(ball_sack) {
          if old_wins.len() == len - 1 {
            let (v1, v2) = board.matched(ball_sack);
            println!("{:?}", v2.iter().sum::<i64>() * ball);
            println!("{:?}:?", ball);
            return
          }
          old_wins.insert(i);
        }
      }
    }
}

pub fn day4_orig() {
    let bingos = get_bingos("data/bingos.txt");
    println!("{:?}", bingos.balls);
    let ball_sack = &mut HashSet::new();
    for ball in bingos.balls {
      ball_sack.insert(ball);
      for  board in &bingos.boards {
        if board.has_bingo(ball_sack) {
          let (v1, v2) = board.matched(ball_sack);
          println!("{:?}", v2.iter().sum::<i64>() * ball);
          println!("{:?}:?", ball);


          return
        }
      }
    
    }

}

    pub fn day4() {
      day4_orig();
      day4_new();

    }
