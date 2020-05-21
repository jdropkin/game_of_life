// Size of edge of board
const BOARD_SIZE: usize = 100;

#[derive(Copy, Clone)]
enum CellState {
    Alive,
    Dead,
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
    board: Vec<Vec<CellState>>,
    rounds: u32,
    alive: u32,
    dead: u32,
}

impl Game {
    // Create a new game
    fn new() -> Self {
        Game {
            board: vec![vec![CellState::Dead; BOARD_SIZE]; BOARD_SIZE],
            rounds: 0,
            alive: 0,
            dead: 0,
        }
    }

    // Kill a cell
    fn kill_cell(&mut self, i: usize, j: usize) {
        self.board[i][j] = CellState::Dead;
    }

    // Revivce a cell
    fn revive_cell(&mut self, i: usize, j: usize) {
        self.board[i][j] = CellState::Alive;
    }

    // Get state of cell
    fn get_state(&self, i: usize, j: usize) -> CellState {
        self.board[i][j]
    }

    // Count living neighbors of a cell
    // This function assumes the board size <= i32::MAX
    fn living_neighbors(board: &Vec<Vec<CellState>>, i: i32, j: i32) -> usize {
        let mut cnt: usize = 0;

        // Loop through columns
        for n in -1..1 {
            let col = n + i;
            // Check for out of bounds
            if col < 0 || col >= BOARD_SIZE as i32 {
                continue;
            }
            // Loop through rows
            for m in -1..1 {
                let row = m + j;
                // Check for out of bounds
                if row < 0 || row >= BOARD_SIZE as i32 {
                    continue;
                }
                // If current cell is alive, increment counter
                match board[col as usize][row as usize] {
                    CellState::Alive => cnt += 1,
                    _ => (),
                }
            }
        }

        cnt
    }

    // Iterate a step in the game
    fn step(mut self) {
        for (i, row) in &mut self.board.iter().enumerate() {
            for (j, _col) in row.iter().enumerate() {
                let neighbors = Game::living_neighbors(&self.board, i as i32, j as i32);

                match self.get_state(i, j) {
                    CellState::Alive => {
                        match neighbors {
                            2 | 3 => (),
                            _ => self.kill_cell(i, j),
                        }
                    },
                    CellState::Dead => {
                        if neighbors == 3 {
                            self.revive_cell(i, j);
                        }
                    },
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
