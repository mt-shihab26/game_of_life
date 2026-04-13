use std::io::Result;

use ratatui::crossterm::event;

fn main() -> Result<()> {
    ratatui::run(|terminal| -> Result<()> {
        loop {
            terminal.draw(|frame| {
                frame.render_widget("hello world", frame.area());
            })?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })?;

    Ok(())
}
