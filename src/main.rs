use std::{
    io::{self, stdin, stdout, Write},
    sync::mpsc::channel,
    thread::spawn,
    time::Duration,
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

enum UserEvent {
    Exit,
    Generate,
}

fn main() -> io::Result<()> {
    // set up terminal
    let mut screen = AlternateScreen::from(stdout().into_raw_mode()?);
    write!(screen, "{}", cursor::Hide)?;

    // welcome user
    write_welcome_msg(&mut screen);
    screen.flush()?;

    // get stream of keystrokes from user
    let (tx, rx) = channel();
    spawn(move || {
        for evt in stdin().keys() {
            match evt.unwrap() {
                Key::Char('q') => {
                    tx.send(UserEvent::Exit).unwrap();
                }
                Key::Char('n') => {
                    tx.send(UserEvent::Generate).unwrap();
                }
                _ => (),
            }
        }
    });

    // build game board
    let (x, y) = termion::terminal_size()?;
    let mut board = board::Board::with_dimensions(x as usize, y as usize);
    board.generate();

    // longer timeout for welcome screen
    let mut frame_rate = Duration::from_secs(3);

    loop {
        let mut should_update = true;

        match rx.recv_timeout(frame_rate) {
            Ok(UserEvent::Exit) => break,
            Ok(UserEvent::Generate) => {
                board.generate();
                should_update = false;
            }
            _ => (),
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

        // shorter for subsequent frames
        frame_rate = Duration::from_millis(250);
    }

    Ok(())
}
