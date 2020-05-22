// Size of edge of board
const BOARD_SIZE: usize = 10;
const VERBOSE: bool = true;
const DEBUG: bool = false;

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

    fn set_cell_state(&mut self, i: usize, j: usize, state: CellState) {
        self.board[i][j] = state;
    }

    fn set_cell_states(&mut self, cells: Vec<(usize, usize)>, state: CellState) {
        for (i, j) in cells {
            self.set_cell_state(i, j, state);
        }
    }

    fn set_cell_state_buf(game: &mut Game, i: usize, j: usize, state: CellState) {
        game.board_buf[i][j] = state;
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
                        cnt += 1;
                    }
                    _ => (),
                }
            }
        }

        cnt
    }

    fn update_cell(game: &mut Game, neighbors: usize, i: usize, j: usize) {
        match game.get_state(i, j) {
            CellState::Alive => match neighbors {
                2 | 3 => Game::set_cell_state_buf(game, i, j, CellState::Alive),
                _ => {
                    Game::set_cell_state_buf(game, i, j, CellState::Dead);
                    game.dead += 1;
                }
            },
            CellState::Dead => {
                if neighbors == 3 {
                    Game::set_cell_state_buf(game, i, j, CellState::Alive);
                    game.alive += 1;
                } else {
                    Game:: set_cell_state_buf(game, i, j, CellState::Dead);
                }
            }
        }
    }

    // Iterate a step in the game
    fn step(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let neighbors = Game::living_neighbors(&self.board, i, j);
                Game::update_cell(self, neighbors, i, j);
            }
        }
        self.rounds += 1;

        // Swap buffer board and board
        std::mem::swap(&mut self.board, &mut self.board_buf);

        if VERBOSE {
            self.print_board();
        }
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    // Print filled in square
                    CellState::Alive => print!("{} ", '\u{25a0}'),
                    // Print empty square
                    CellState::Dead => print!("{} ", '\u{25a1}'),
                }
            }
            println!();
        }
        println!();

        if DEBUG {
            println!("buf:");
            for row in self.board_buf.iter() {
                for cell in row {
                    match cell {
                        // Print filled in square
                        CellState::Alive => print!("{} ", '\u{25a0}'),
                        // Print empty square
                        CellState::Dead => print!("{} ", '\u{25a1}'),
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let mut g = Game::new();
    /*g.set_cell_states(
        vec![(1, 0), (1, 1), (1, 2), (2, 1), (2, 2), (2, 3)],
        CellState::Alive,
    );*/
    g.set_cell_states(
        vec![(2, 1), (3, 2), (3, 3), (2, 3), (1, 3)],
        CellState::Alive,
    );
    g.print_board();
    g.step();
    g.step();
    g.step();
    g.step();
    g.step();
    g.step();
    g.step();
    g.step();
}
