use std::{io::Result, time::Duration};

use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};

fn main() -> Result<()> {
    let (cols, rows) = terminal::size()?;

    let mut cells = vec![vec![0; cols as usize]; rows as usize];

    set_start(&mut cells);

    ratatui::run(|terminal| -> Result<()> {
        loop {
            game_of_life(&mut cells);

            if event::poll(Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(e) => match e.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    _ => {}
                }
            }
            terminal.draw(|frame| {
                let buf = frame.buffer_mut();

                for (y, row) in cells.iter().enumerate() {
                    for (x, &value) in row.iter().enumerate() {
                        let symbol = if value == 0 { " " } else { "█" };
                        if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
                            cell.set_symbol(symbol);
                        }
                    }
                }
            })?;
        }
    })?;

    Ok(())
}

fn set_start(cells: &mut Vec<Vec<i32>>) {
    cells[1][2] = 1;
    cells[2][0] = 1;
    cells[2][2] = 1;
    cells[3][1] = 1;
    cells[3][2] = 1;
}

//123
//4x5
//678

fn game_of_life(cells: &mut Vec<Vec<i32>>) {
    let cloned = cells.clone();

    for (y, row) in cells.iter_mut().enumerate() {
        for (x, value) in row.iter_mut().enumerate() {
            let live_neighbors = get(&cloned, x - 1, y - 1)
                + get(&cloned, x, y - 1)
                + get(&cloned, x + 1, y - 1)
                + get(&cloned, x - 1, y)
                + get(&cloned, x + 1, y)
                + get(&cloned, x - 1, y + 1)
                + get(&cloned, x, y + 1)
                + get(&cloned, x + 1, y + 1);

            *value = live_neighbors;
        }
    }
}

fn get(cells: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    match cells.get(y) {
        Some(row) => match row.get(x) {
            Some(value) => *value,
            None => 0,
        },
        None => 0,
    }
}
