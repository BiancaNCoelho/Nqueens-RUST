const N: usize = 13;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Clone)]
struct Board([[bool; N]; N]);

fn tenta(board: &mut Board, row: usize, count: &AtomicI64) {
    if row == N {
        count.fetch_add(1, Ordering::Relaxed);
        // Faz print de cada solução possivel do N-Rainhas
        //println!("Solução #{}:\n", *count);
        //for r in board.iter() {
        //    println!(
         //       "{}",
         //       r.iter()
         //           .map(|&x| if x { "Q" } else { "." }.to_string())
          //          .collect::<Vec<String>>()
          //          .join(" ")
          //  )
        //}
        //println!("");
        return;
    }
    // Verifica posições das rainhas, se são inválidas ou não
    //for i in 0..N {
    (0..N).into_par_iter().for_each(|i|{
    	let mut new_board = board.clone();
        let mut valid_move: bool = true;
        for j in 0..row {   
            if new_board.0[j][i]
                || i + j >= row && new_board.0[j][i + j - row]
                || i + row < N + j && new_board.0[j][i + row - j]
            {
                 valid_move = false;
                 break;
            }
        }
        if valid_move {
            new_board.0[row][i] = true;
            tenta(&mut new_board, row + 1, count);
           new_board.0[row][i] = false;
        }
    }
    );
}

fn main() {
    let mut board = Board([[false; N]; N]); // tabuleiro
    let count = AtomicI64::new(0);// contador para contagem de soluções possíveis
    tenta(&mut board, 0, &count);
    println!("Soluções : {}", count.load(Ordering::Relaxed));
}
