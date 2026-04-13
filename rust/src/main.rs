use std::{io::Result, time::Duration};

use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};

fn main() -> Result<()> {
    let (cols, rows) = terminal::size()?;

    let mut cells = vec![vec![0; cols as usize]; rows as usize];

    ratatui::run(|terminal| -> Result<()> {
        loop {
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

                for (x, row) in cells.iter().enumerate() {
                    for (y, col) in row.iter().enumerate() {
                        if x * y % 10 == 0 {
                            if let Some(cell) = buf.cell_mut((x, y)) {
                                cell.set_symbol("█");
                            }
                        }
                    }
                }
            })?;
        }
    })?;

    Ok(())
}
