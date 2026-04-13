use std::{io::Result, time::Duration};

use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};

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

            let (width, height) = terminal::size()?;

            terminal.draw(|frame| {
                for i in 1..width {
                    for j in 1..height {
                        frame.render_widget("█", frame.area());
                    }
                }
            })?;
        }
    })?;

    Ok(())
}
