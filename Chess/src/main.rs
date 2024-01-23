use std::io;

// pieces
const KING_WHITE: char = '\u{2654}';
const QUEEN_WHITE: char = '\u{2655}';
const ROOK_WHITE: char = '\u{2656}';
const BISHOP_WHITE: char = '\u{2657}';
const KNIGHT_WHITE: char = '\u{2658}';
const PAWN_WHITE: char = '\u{2659}';


const KING_BLACK: char = '\u{265A}';
const QUEEN_BLACK: char = '\u{265B}';
const ROOK_BLACK: char = '\u{265C}';
const BISHOP_BLACK: char = '\u{265D}';
const KNIGHT_BLACK: char = '\u{265E}';
const PAWN_BLACK: char = '\u{265F}';

const EMPTY_CASE: char = '·';


fn show_board(board: &mut Vec<Vec<char>>) {
    print!(" ");
    for i in 0..8 {
        print!("{} ", (b'a'+i) as char);
    }
    println!("");
    
    for i in 0..8 {
        print!("{}", (i+1));
        for j in 0..8 {
            print!("{} ", board[i][j]);
            
        }
        println!("");
    }
}

fn init_board(board: &mut Vec<Vec<char>>){
    board[0][0] = ROOK_BLACK as char;
    board[0][1] = KNIGHT_BLACK as char;
    board[0][2] = BISHOP_BLACK as char;
    board[0][3] = QUEEN_BLACK as char;
    board[0][4] = KING_BLACK as char;
    board[0][5] = BISHOP_BLACK as char;
    board[0][6] = KNIGHT_BLACK as char;
    board[0][7] = ROOK_BLACK as char;

    board[7][0] = ROOK_WHITE as char;
    board[7][1] = KNIGHT_WHITE as char;
    board[7][2] = BISHOP_WHITE as char;
    board[7][3] = QUEEN_WHITE as char;
    board[7][4] = KING_WHITE as char;
    board[7][5] = BISHOP_WHITE as char;
    board[7][6] = KNIGHT_WHITE as char;
    board[7][7] = ROOK_WHITE as char;

    for i in 0..8 {
        board[1][i] = PAWN_BLACK as char;
        board[6][i] = PAWN_WHITE as char;
    }

    for i in 2..6 {
        for j in 0..8 {
            board[i][j] = EMPTY_CASE;
        }
    }
    
}

fn move_piece(board: &mut Vec<Vec<char>>, from: &str, to: &str) {

    if check_move(from,to) == false{
        println!("Invalid move");
        return;
    }

    let from_x = (from.as_bytes()[0] - b'a') as usize;
    let from_y = (from.as_bytes()[1] - b'1') as usize;
    let to_x = (to.as_bytes()[0] - b'a') as usize;
    let to_y = (to.as_bytes()[1]- b'1' ) as usize;

    board[to_y][to_x] = board[from_y][from_x];
    board[from_y][from_x] = EMPTY_CASE;
}

fn check_move(from: &str, to: &str)->bool{
    let from_x = (from.as_bytes()[0] - b'a') as i32;
    let from_y = (from.as_bytes()[1] - b'1') as i32;
    let to_x = (to.as_bytes()[0] - b'a') as i32;
    let to_y = (to.as_bytes()[1]- b'1' ) as i32;

    if from_x > 7 || from_y > 7 || to_x > 7 || to_y > 7 {
        return false;
    }
    return true;
}


fn main(){
    let mut board: Vec<Vec<char>> = vec![vec![' '; 8]; 8];

    init_board(&mut board);
    show_board(&mut board);

    loop{
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line");

            
        instruction = instruction.trim().to_string().to_lowercase();
    
        move_piece(&mut board, &instruction[0..2], &instruction[2..4]);
        show_board(&mut board);
    }
    
}
