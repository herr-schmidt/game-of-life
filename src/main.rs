use std::io::{stdout, Stdout, Write};
use crossterm::{
    cursor, terminal, ExecutableCommand,
};
use rand::{self, Rng};

const ROWS: usize = 20;
const COLUMNS: usize = 80;

fn main() {
    let mut table = [[1; COLUMNS]; ROWS];

    // lock stdout and use the same locked instance throughout
    let mut stdout = stdout();
    initialize_table(&mut table);

    update_gui(&mut stdout, &mut table);
    for i in 0..500 {
        advance_generation(&mut table);
        std::thread::sleep(std::time::Duration::from_millis(25));
        update_gui(&mut stdout, &mut table);
    }
}

fn initialize_table(table: &mut [[i32; COLUMNS]; ROWS]) {
    let mut rng = rand::thread_rng();

    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let dead_or_alive = rng.gen_range(0..=9);
            if dead_or_alive == 0 {
                table[row][column] = 1;
            } else {
                table[row][column] = 0;
            }
        }
    }
}

fn update_gui(stdout: &mut Stdout, table: &mut [[i32; COLUMNS]; ROWS]) {
    // move cursor up and then clear terminal
    stdout.execute(cursor::MoveUp(ROWS as u16)).expect("TODO: panic message");
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("TODO: panic message");

    for row in table {
        let row_as_string = row.iter().map(|cell| {
            let mut cell_char = " ";
            if *cell == 1 {
                cell_char = "■"
            }
            format!("{}", cell_char)
        })
            .collect::<Vec<String>>().join(" ");
        writeln!(stdout, "{}", row_as_string).expect("TODO: panic message");
    }
}


fn advance_generation(table: &mut [[i32; COLUMNS]; ROWS]) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let neighbourhood = get_cell_neighbourhood(table, row, column);
            let neighbourhood_sum: i32 = neighbourhood.iter().sum();
            if table[row][column] == 0 {
                if neighbourhood_sum == 3 {
                    table[row][column] = 1;
                }
            } else {
                if neighbourhood_sum < 2 || neighbourhood_sum > 3 {
                    table[row][column] = 0;
                }
            }
        }
    }
}

fn get_cell_neighbourhood(table: &mut [[i32; COLUMNS]; ROWS], row: usize, column: usize) -> Vec<i32> {
    const LAST_COLUMN: usize = COLUMNS - 1;
    const LAST_ROW: usize = ROWS - 1;
    match (row, column) {
        // corners
        (0, 0) => vec![table[0][1],
                       table[1][0],
                       table[1][1]],
        (0, LAST_COLUMN) => vec![table[0][column - 1],
                                 table[1][column - 1],
                                 table[1][column]],
        (LAST_ROW, 0) => vec![table[row - 1][0],
                              table[row - 1][1],
                              table[row][1]],
        (LAST_ROW, LAST_COLUMN) => vec![table[row - 1][column],
                                        table[row - 1][column - 1],
                                        table[row][column - 1]],
        // borders (no corners)
        (0, _) => vec![table[row][column - 1],
                       table[row][column + 1],
                       table[row + 1][column - 1],
                       table[row + 1][column],
                       table[row + 1][column + 1]],
        (LAST_ROW, _) => vec![table[row][column - 1],
                              table[row][column + 1],
                              table[row - 1][column - 1],
                              table[row - 1][column],
                              table[row - 1][column + 1]],
        (_, 0) => vec![table[row - 1][column],
                       table[row - 1][column + 1],
                       table[row][column + 1],
                       table[row + 1][column],
                       table[row + 1][column + 1]],
        (_, LAST_COLUMN) => vec![table[row - 1][column],
                                 table[row - 1][column - 1],
                                 table[row][column - 1],
                                 table[row + 1][column],
                                 table[row + 1][column - 1]],
        // elsewhere
        _ => vec![table[row - 1][column - 1],
                  table[row - 1][column],
                  table[row - 1][column + 1],
                  table[row][column - 1],
                  table[row][column + 1],
                  table[row + 1][column - 1],
                  table[row + 1][column],
                  table[row + 1][column + 1]],
    }
}

