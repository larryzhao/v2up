use crate::context::Context;
use crate::v2ray::server::Server;
use std::{io, sync::mpsc, thread, time::Duration};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

pub fn exec<B: Backend>(
    ctx: &mut Context,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(&ctx.settings.subscriptions[0].servers);
    app.run(terminal)
}

struct App<'a> {
    servers: StatefulList<&'a str>,
    tick_rate: Duration,
}

impl<'a> App<'a> {
    fn new(servers: &'a Vec<Server>) -> Self {
        let mut items = vec![];
        for server in servers {
            match server {
                Server::Vmess(server) => items.push(server.name.as_str()),
            }
        }

        return App {
            servers: StatefulList::with_items(items),
            tick_rate: Duration::from_millis(250),
        };
    }
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80)].as_ref())
            .split(f.size());

        // Iterate through all elements in the `items` app and append some debug text to it.
        let items: Vec<ListItem> = self
            .servers
            .items
            .iter()
            .map(|server| {
                let mut lines = vec![Spans::from(*server)];
                //         lines.push(Spans::from(Span::styled(
                // "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                // Style::default().add_modifier(Modifier::ITALIC),
                // let mut lines = vec![Spans::from(i.0)];
                // for _ in 0..i.1 {

                //     )));
                // }
                ListItem::new(lines).style(Style::default().fg(Color::Cyan))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, chunks[0], &mut self.servers.state);
    }

    fn on_tick(&self) {}

    fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let events = events(self.tick_rate);
        loop {
            terminal.draw(|f| self.draw(f))?;
            match events.recv()? {
                Event::Input(key) => match key {
                    Key::Char('q') => return Ok(()),
                    Key::Left => self.servers.unselect(),
                    Key::Down => self.servers.next(),
                    Key::Up => self.servers.previous(),
                    _ => {}
                },
                Event::Tick => self.on_tick(),
            }
        }
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

enum Event {
    Input(Key),
    Tick,
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Input(key)) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}
