use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    Black,
    White,
}
impl Player {
    fn opposite(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Player::Black => '●',
            Player::White => '○',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

impl Cell {
    fn symbol(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Occupied(player) => player.symbol(),
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: [[Cell; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            cells: [[Cell::Empty; 8]; 8],
        };

        // 初期配置
        board.cells[3][3] = Cell::Occupied(Player::White);
        board.cells[3][4] = Cell::Occupied(Player::Black);
        board.cells[4][3] = Cell::Occupied(Player::Black);
        board.cells[4][4] = Cell::Occupied(Player::White);

        board
    }

    fn display(&self) {
        println!("  a b c d e f g h");
        for (row, line) in self.cells.iter().enumerate() {
            print!("{} ", row + 1);
            for cell in line {
                print!("{} ", cell.symbol());
            }
            println!();
        }
    }

    fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < 8 && col < 8
    }

    fn is_valid_move(&self, row: usize, col: usize, player: Player) -> bool {
        if !self.is_valid_position(row, col) || self.cells[row][col] != Cell::Empty {
            return false;
        }

        // 8方向をチェック
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.check_direction(row, col, *dr, *dc, player) {
                return true;
            }
        }

        false
    }

    fn check_direction(&self, row: usize, col: usize, dr: i32, dc: i32, player: Player) -> bool {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        let mut found_opponent = false;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match self.cells[r as usize][c as usize] {
                Cell::Empty => return false,
                Cell::Occupied(p) if p == player => return found_opponent,
                Cell::Occupied(_) => {
                    found_opponent = true;
                    r += dr;
                    c += dc;
                }
            }
        }

        false
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) -> bool {
        if !self.is_valid_move(row, col, player) {
            return false;
        }

        self.cells[row][col] = Cell::Occupied(player);

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.check_direction(row, col, *dr, *dc, player) {
                self.flip_pieces(row, col, *dr, *dc, player);
            }
        }

        true
    }

    fn flip_pieces(&mut self, row: usize, col: usize, dr: i32, dc: i32, player: Player) {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            match self.cells[r as usize][c as usize] {
                Cell::Occupied(p) if p == player => break,
                Cell::Occupied(_) => {
                    self.cells[r as usize][c as usize] = Cell::Occupied(player);
                    r += dr;
                    c += dc;
                }
                Cell::Empty => break,
            }
        }
    }

    fn count_pieces(&self, player: Player) -> u32 {
        let mut count = 0;
        for row in &self.cells {
            for cell in row {
                if let Cell::Occupied(p) = cell {
                    if *p == player {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn has_valid_moves(&self, player: Player) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if self.is_valid_move(row, col, player) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct Game {
    board: Board,
    current_player: Player,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Player::Black,
            game_over: false,
        }
    }

    fn play(&mut self) {
        println!("オセロゲームへようこそ！");
        println!("黒（●）が先手です。");
        println!("座標は a1-h8 の形式で入力してください（例: d3）");
        println!();

        loop {
            self.board.display();
            println!();
            println!(
                "黒: {} 個, 白: {} 個",
                self.board.count_pieces(Player::Black),
                self.board.count_pieces(Player::White)
            );

            if self.game_over {
                self.show_final_result();
                break;
            }

            if !self.board.has_valid_moves(self.current_player) {
                println!(
                    "{}のプレイヤーは打てる場所がありません。パスします。",
                    if self.current_player == Player::Black {
                        "黒"
                    } else {
                        "白"
                    }
                );

                self.current_player = self.current_player.opposite();

                if !self.board.has_valid_moves(self.current_player) {
                    self.game_over = true;
                    continue;
                }
                continue;
            }

            println!(
                "{}のターンです。座標を入力してください: ",
                if self.current_player == Player::Black {
                    "黒（●）"
                } else {
                    "白（○）"
                }
            );

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("入力エラーです。");
                continue;
            }

            let input = input.trim().to_lowercase();
            if input == "quit" || input == "exit" {
                break;
            }

            if let Some((row, col)) = self.parse_input(&input) {
                if self.board.make_move(row, col, self.current_player) {
                    self.current_player = self.current_player.opposite();
                } else {
                    println!("無効な手です。もう一度入力してください。");
                }
            } else {
                println!("無効な形式です。a1-h8の形式で入力してください。");
            }
            println!();
        }
    }

    fn parse_input(&self, input: &str) -> Option<(usize, usize)> {
        if input.len() != 2 {
            return None;
        }

        let chars: Vec<char> = input.chars().collect();
        let col = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };

        let row = match chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        Some((row, col))
    }

    fn show_final_result(&self) {
        println!("=== ゲーム終了 ===");
        self.board.display();
        println!();

        let black_count = self.board.count_pieces(Player::Black);
        let white_count = self.board.count_pieces(Player::White);

        println!("最終結果:");
        println!("黒（●）: {} 個", black_count);
        println!("白（○）: {} 個", white_count);

        if black_count > white_count {
            println!("黒の勝利です！");
        } else if white_count > black_count {
            println!("白の勝利です！");
        } else {
            println!("引き分けです！");
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
