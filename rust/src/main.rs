use std::{io::Result, time::Duration};

use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};

fn main() -> Result<()> {
    let (cols, rows) = terminal::size()?;

    let cells = vec![vec![0; cols as usize]; rows as usize];

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
