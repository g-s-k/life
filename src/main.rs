use std::{
    io::{self, stdin, stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::*};

mod board;

fn write_welcome_msg<W: Write>(screen: &mut W) {
    write!(
        screen,
        concat!(
            "{}{}Welcome to the Game of Life.",
            "{}Press the space bar to start or pause the game.",
            "{}Press N to discard the current game and generate a new one.",
            "{}Press Q to exit."
        ),
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 3),
        termion::cursor::Goto(1, 4),
        termion::cursor::Goto(1, 5),
    )
    .unwrap();
}

fn main() -> io::Result<()> {
    // set up terminal
    let mut screen = AlternateScreen::from(stdout().into_raw_mode()?);
    write!(screen, "{}", cursor::Hide)?;

    // welcome user
    write_welcome_msg(&mut screen);
    screen.flush()?;

    // get stream of keystrokes from user
    let mut events = stdin().keys().peekable();

    // build game board
    let (x, y) = termion::terminal_size()?;
    let mut board = board::Board::with_dimensions(x as usize, y as usize);
    board.generate();

    'main: loop {
        let mut should_update = true;
        let waiting_since = Instant::now();

        'blocking: while waiting_since.elapsed() < Duration::from_millis(500) {
            if events.peek().is_some() {
                match events.next().unwrap()? {
                    Key::Char('q') => break 'main,
                    Key::Char('n') => {
                        board.generate();
                        should_update = false;
                        break 'blocking;
                    }
                    _ => (),
                }
            } else {
                sleep(Duration::from_millis(50));
            }
        }

        write!(
            screen,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            board
        )?;
        screen.flush()?;

        if should_update {
            board.update();
        }
    }

    Ok(())
}
