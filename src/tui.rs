// imports {{{

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind},
    queue, style, terminal,
};

use std::io::{Stdout, Write};
use std::time::{Duration, Instant};

pub use super::cells::{Game, Grid};

// imports }}}

// TuiOpts {{{
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TuiOpts {
    pub numeric: bool,
    pub monochrome: bool,
}
// TuiOpts }}}

// State {{{
#[derive(Clone)]
struct State {
    opts: TuiOpts,
    game: Game,
    xy: (u16, u16),
    update: bool,
    advance: bool,
    quit: bool,
}
// State }}}

// try_read {{{
/// blocks on negative secs
fn try_read(secs: f32) -> Option<Event> {
    if secs.is_sign_positive() {
        event::poll(Duration::from_secs_f32(secs))
            .ok()
            .map(|b| b.then(|| event::read().ok()))
            .flatten()
            .flatten()
    } else {
        event::read().ok()
    }
}
// try_read }}}

// draw {{{
fn draw(stdout: &mut Stdout, state: &State) {
    queue!(stdout, cursor::SavePosition, cursor::MoveTo(0, 0)).expect("Cursor move fail");

    let grid = state.game.grid();

    for (n, row) in grid.iter().enumerate() {
        for cell in row {
            queue!(
                stdout,
                style::Print(if *cell == 0 {
                    ' '.to_string()
                } else if state.opts.numeric {
                    if state.game.opts().life > 9 {
                        (((*cell as f32 / state.game.opts().life as f32) * 9.0).round() as u8)
                            .min(9)
                            .to_string()
                    } else {
                        cell.to_string()
                    }
                } else {
                    '█'.to_string()
                })
            )
            .expect("print cell fail")
        }
        if n != grid.len() {
            queue!(stdout, cursor::MoveTo(0, 1 + n as u16)).expect("Cursor move fail");
        };
    }

    queue!(stdout, cursor::RestorePosition).expect("Cursor move fail");

    stdout.flush().expect("Terminal flush fail");
}
// draw }}}

// process_event {{{
fn process_event(state: &mut State, event: Event) {
    match event {
        Event::Key(kevt) => match kevt.code {
            KeyCode::Char('n') => {
                state.game.advance();
                state.update = true
            }
            KeyCode::Char('a') => {
                state.advance = !state.advance;
            }
            KeyCode::Esc => state.quit = true,
            KeyCode::Char('c') if kevt.modifiers.contains(KeyModifiers::CONTROL) => {
                state.quit = true
            }
            _ => (),
        },
        Event::Mouse(mevt) => match mevt.kind {
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) => {
                let val = state.game.opts().life;
                if state.game.get_cell(mevt.column.into(), mevt.row.into()) != Some(val) {
                    state
                        .game
                        .set_cell(mevt.column.into(), mevt.row.into(), val);
                    state.update = true;
                }
            }
            MouseEventKind::Drag(MouseButton::Right) | MouseEventKind::Down(MouseButton::Right) => {
                if state.game.get_cell(mevt.column.into(), mevt.row.into()) != Some(0) {
                    state.game.set_cell(mevt.column.into(), mevt.row.into(), 0);
                    state.update = true;
                }
            }
            _ => (),
        },
        Event::Resize(x, y) => {
            state.xy = (x, y);
            state.game.resize(x.into(), y.into());
            state.update = true;
        }
        _ => (),
    }
}
// process_event }}}

// run {{{
pub fn run(game: Game, opts: TuiOpts) {
    let mut stdout = std::io::stdout();

    // Initialize
    terminal::enable_raw_mode().expect("Terminal could not enter raw");

    let mut state = State {
        game,
        opts,
        xy: terminal::size().expect("Could not query terminal size"),
        update: false,
        advance: false,
        quit: false,
    };

    state.game.resize(state.xy.0.into(), state.xy.1.into());

    queue!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture,
        terminal::DisableLineWrap,
        cursor::Hide,
        cursor::MoveTo(state.xy.0 / 2, state.xy.1 / 2)
    )
    .expect("Terminal init fail");

    draw(&mut stdout, &mut state);

    let mut draw_times = vec![];
    // Main loop
    while !state.quit {
        if let Some(evt) = try_read(0.1) {
            process_event(&mut state, evt)
        } else if state.advance {
            state.game.advance();
            state.update = true;
        }

        if state.update {
            let dt = Instant::now();
            draw(&mut stdout, &mut state);
            state.update = false;
            draw_times.push(Instant::now() - dt);
        }
    }

    // Cleanup
    queue!(
        stdout,
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture,
        cursor::Show,
        terminal::EnableLineWrap
    )
    .expect("Terminal cleanup fail");
    terminal::disable_raw_mode().expect("Terminal could not exit raw");
    stdout.flush().expect("Terminal flush fail");

    println!(
        "DRAW_AVG: {}",
        (draw_times.iter().sum::<Duration>() / draw_times.len() as u32).as_millis()
    );
    println!(
        "DRAW_MEDIAN: {}",
        draw_times[draw_times.len() / 2].as_millis()
    )
}
// run }}}
