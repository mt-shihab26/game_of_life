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

            if event::poll(Duration::from_millis(120))? {
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
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if rand::random::<f32>() > 0.5 { 1 } else { 0 };
        }
    }
}

fn game_of_life(cells: &mut Vec<Vec<i32>>) {
    let cloned = cells.clone();

    for (y, row) in cells.iter_mut().enumerate() {
        for (x, value) in row.iter_mut().enumerate() {
            let x = x as isize;
            let y = y as isize;

            let live_neighbors = get(&cloned, x - 1, y - 1)
                + get(&cloned, x, y - 1)
                + get(&cloned, x + 1, y - 1)
                + get(&cloned, x - 1, y)
                + get(&cloned, x + 1, y)
                + get(&cloned, x - 1, y + 1)
                + get(&cloned, x, y + 1)
                + get(&cloned, x + 1, y + 1);

            *value = match (*value, live_neighbors) {
                (1, 2) | (1, 3) => 1,
                (0, 3) => 1,
                _ => 0,
            }
        }
    }
}

fn get(cells: &Vec<Vec<i32>>, x: isize, y: isize) -> i32 {
    if y < 0 || x < 0 {
        return 0;
    }

    let y = y as usize;
    let x = x as usize;

    match cells.get(y) {
        Some(row) => match row.get(x) {
            Some(value) => *value,
            None => 0,
        },
        None => 0,
    }
}
