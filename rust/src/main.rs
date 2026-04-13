use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event},
    style::{self, Stylize},
    terminal,
};
use std::{
    io::{self, Write},
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(event) => match event.code {
                    event::KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => (),
                },
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                _ => (),
            }
        }

        stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::PrintStyledContent("Hello World".magenta()))?;

        stdout.flush()?;
    }
}
