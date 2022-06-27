
use std::io;
use std::process::exit;
use figlet_rs::*;
use colored::*;
use rand::*;
use term_table::{TableBuilder, TableStyle};
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

enum Winners {
    X,
    O,
    Draw,
    Undecided
}

fn logic() {
    let mut board_vec: Vec<char> = vec![' ',' ',' ',' ',' ',' ',' ',' ',' '];
    loop {
        draw_table_board(&mut board_vec);
        let winner: Winners = win_detect(&mut board_vec);
        match winner {
            Winners::X => {
                let standard_font = FIGfont::standand().unwrap();
                let x_winner = standard_font.convert("X Wins").unwrap();
                println!("\n{}", x_winner.to_string().green());
                break;
            },
            Winners::O => {
                let standard_font = FIGfont::standand().unwrap();
                let o_winner = standard_font.convert("O Wins").unwrap();
                println!("\n{}", o_winner.to_string().cyan());
                break;
            },
            Winners::Draw => {
                let standard_font = FIGfont::standand().unwrap();
                let drawer = standard_font.convert("It\'s a Draw!").unwrap();
                println!("\n{}", drawer.to_string().magenta());
                break;
            },
            _ => ()
        };
        let mut input_text= String::new();
        println!("\nEnter a number from 1 to 9: \r");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let parsed: usize = match input_text.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error!");
                break;
            }
        };
        if parsed.to_string().trim().parse::<usize>().is_ok() {
            if parsed <= board_vec.len() && parsed > 0 && board_vec[(parsed - 1) as usize] == ' ' {
                board_vec[parsed - 1] = 'X';
                loop {
                    let rnd_num: i32 = thread_rng().gen_range(0..8);
                    if board_vec[rnd_num as usize] == ' ' {
                        board_vec[rnd_num as usize] = 'O';
                        break;
                    }
                    else if !board_vec.contains(&' ') {
                        break;
                    }
                    else {
                        continue;
                    }
                }
            }
            else {
                println!("Number out of range or its invalid!")
            }
        }
        else {
            println!("Not a number!");
        }
    }
}

fn draw_table_board(board: &mut Vec<char>) {
    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new_with_alignment(board[0], 1, Alignment::Left),
                TableCell::new_with_alignment(board[1], 1, Alignment::Center),
                TableCell::new_with_alignment(board[2], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[3], 1, Alignment::Left),
                TableCell::new_with_alignment(board[4], 1, Alignment::Center),
                TableCell::new_with_alignment(board[5], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[6], 1, Alignment::Left),
                TableCell::new_with_alignment(board[7], 1, Alignment::Center),
                TableCell::new_with_alignment(board[8], 1, Alignment::Right)
            ]),
        ]
    ).build();

    println!("\n{}\n", table.render());
}

fn win_detect(board: &mut Vec<char>) -> Winners {
    return if (board[0] == 'X' && board[1] == 'X' && board[2] == 'X') || (board[3] == 'X' && board[4] == 'X' && board[5] == 'X') || (board[6] == 'X' && board[7] == 'X' && board[8] == 'X') || (board[0] == 'X' && board[3] == 'X' && board[6] == 'X') || (board[1] == 'X' && board[4] == 'X' && board[7] == 'X') || (board[2] == 'X' && board[5] == 'X' && board[8] == 'X') || (board[0] == 'X' && board[4] == 'X' && board[8] == 'X') || (board[2] == 'X' && board[4] == 'X' && board[6] == 'X') {
        Winners::X
    } else if (board[0] == 'O' && board[1] == 'O' && board[2] == 'O') || (board[3] == 'O' && board[4] == 'O' && board[5] == 'O') || (board[6] == 'O' && board[7] == 'O' && board[8] == 'O') || (board[0] == 'O' && board[3] == 'O' && board[6] == 'O') || (board[1] == 'O' && board[4] == 'O' && board[7] == 'O') || (board[2] == 'O' && board[5] == 'O' && board[8] == 'O') || (board[0] == 'O' && board[4] == 'O' && board[8] == 'O') || (board[2] == 'O' && board[4] == 'O' && board[6] == 'O') {
        Winners::O
    } else if !board.contains(&' ') {
        Winners::Draw
    } else {
        Winners::Undecided
    }
}

pub fn main_game() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("TicTacToe").unwrap();
    println!("{}", figure.to_string().green());
    println!("\nWelcome to command-line tictactoe! The board will be shown in the commands line after every turn and you will have to enter a number ranging from 1 to 9 in order to place your sign (X) into that specific cell of the board. The computer will make a random move.\n");
    loop {
        logic();
        println!("\n\nContinue playing? (y/n): ");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Invalid input!");
        if response.contains(&"n") {
            println!("{}", "Exiting program ...".to_string().red());
            exit(0);
        }
        else {
            continue;
        }
    }
}