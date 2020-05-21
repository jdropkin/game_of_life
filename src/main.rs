// Size of edge of board
const BOARD_SIZE: usize = 10;
const VERBOSE: bool = true;

#[derive(Copy, Clone)]
enum CellState {
    Alive,
    Dead,
}

type Board = Vec<Vec<CellState>>;

trait GameOfLife {
    fn get_state(&self, i: usize, j: usize) -> CellState;
}

impl GameOfLife for Board {
    fn get_state(&self, i: usize, j: usize) -> CellState {
        self[i][j]
    }
}

/*
 * Struct to represent the current state in game of life
 *
 * board  - the current board of cells
 * rounds - the number of rounds that have happened
 * alive  - the number of living cells
 * dead   - the number of dead cells
 */
struct Game {
    board: Board,
    board_buf: Board,
    rounds: usize,
    alive: usize,
    dead: usize,
}

impl Game {
    // Create a new game
    fn new() -> Self {
        Game {
            board: vec![vec![CellState::Dead; BOARD_SIZE]; BOARD_SIZE],
            board_buf: vec![vec![CellState::Dead; BOARD_SIZE]; BOARD_SIZE],
            rounds: 0,
            alive: 0,
            dead: BOARD_SIZE * BOARD_SIZE,
        }
    }

    // Kill a cell
    fn kill_cell(&mut self, i: usize, j: usize) {
        self.board[i][j] = CellState::Dead;
    }

    fn buf_kill_cell(game: &mut Game, i: usize, j: usize) {
        game.board_buf[i][j] = CellState::Dead;
    }

    // Revivce a cell
    fn revive_cell(&mut self, i: usize, j: usize) {
        self.board[i][j] = CellState::Alive;
    }

    fn buf_revive_cell(game: &mut Game, i: usize, j: usize) {
        game.board_buf[i][j] = CellState::Alive;
    }

    // Get state of cell
    fn get_state(&self, i: usize, j: usize) -> CellState {
        self.board[i][j]
    }

    // Count living neighbors of a cell
    // This function assumes the board size <= i32::MAX
    fn living_neighbors(board: &Vec<Vec<CellState>>, i: usize, j: usize) -> usize {
        let mut cnt: usize = 0;

        // Loop through rows
        for n in -1..2 {
            let row = n + i as i32;
            // Check for out of bounds
            if row < 0 || row >= BOARD_SIZE as i32 {
                continue;
            }
            // Loop through cols
            for m in -1..2 {
                let col = m + j as i32;
                // Check for out of bounds
                if (col < 0) || (col >= BOARD_SIZE as i32) || (n == 0 && m == 0) {
                    continue;
                }
                // If current cell is alive, increment counter
                match board[row as usize][col as usize] {
                    CellState::Alive => {
                        //println!("row: {} col: {}", row, col);
                        cnt += 1;
                    },
                    _ => (),
                }
            }
        }

        cnt
    }

    fn update_cell(game: &mut Game, neighbors: usize, i: usize, j: usize) {
        match game.get_state(i, j) {
            CellState::Alive => match neighbors {
                2 | 3 => Game::buf_revive_cell(game, i, j),
                _ => {
                    Game::buf_kill_cell(game, i, j);
                    game.dead += 1;
                }
            },
            CellState::Dead => {
                if neighbors == 3 {
                    Game::buf_revive_cell(game, i, j);
                    game.alive += 1;
                }
            }
        }
    }

    // Iterate a step in the game
    fn step(mut self) -> Self {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let neighbors = Game::living_neighbors(&self.board, i, j);
                Game::update_cell(&mut self, neighbors, i, j);
            }
        }
        self.rounds += 1;

        // Swap buffer board and board
        let b1 = self.board;
        let b2 = self.board_buf;
        self.board = b2;
        self.board_buf = b1;

        if VERBOSE {
            self.print_board();
        }

        self
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    CellState::Alive => print!("{} ", '\u{25a0}'),
                    CellState::Dead => print!("{} ", '\u{25a1}'),
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut g = Game::new();
    g.revive_cell(0, 1);
    g.revive_cell(1, 0);
    g.revive_cell(1, 2);
    g.revive_cell(2, 1);
    g.print_board();
    g = g.step();
    g = g.step();
}
