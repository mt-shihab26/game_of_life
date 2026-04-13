use std::{io::Result, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode};

fn main() -> Result<()> {
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
                let area = frame.area();
                let buf = frame.buffer_mut();

                for x in 0..area.width {
                    for y in 0..area.height {
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
