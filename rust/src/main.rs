use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event},
    style::{self, Stylize},
    terminal,
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let res = run(&mut stdout);

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    res
}

fn run(stdout: &mut io::Stdout) -> io::Result<()> {
    let mut count = 0;

    let frame_time = Duration::from_millis(16); // ~60 FPS

    loop {
        let frame_start = Instant::now();
        count += 1;

        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(event) => match event.code {
                    event::KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::PrintStyledContent(
                format!("Hello World: {count}").magenta(),
            ))?;

        stdout.flush()?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }
}
