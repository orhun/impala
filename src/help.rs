use std::sync::Arc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, Padding, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
    Frame,
};

use crate::{app::ColorMode, config::Config};

#[derive(Debug)]
pub struct Help {
    block_height: usize,
    state: TableState,
    keys: Vec<(Cell<'static>, &'static str)>,
}

impl Help {
    pub fn new(config: Arc<Config>) -> Self {
        let mut state = TableState::new().with_offset(0);
        state.select(Some(0));

        Self {
            block_height: 0,
            state,
            keys: vec![
                (
                    Cell::from("## Global").style(Style::new().bold().fg(Color::Yellow)),
                    "",
                ),
                (Cell::from("Esc").bold(), "Dismiss help pop-up"),
                (
                    Cell::from("Tab").bold(),
                    "Switch between different sections",
                ),
                (Cell::from("j or Down").bold(), "Scroll down"),
                (Cell::from("k or Up").bold(), "Scroll up"),
                (
                    Cell::from(config.start_scanning.to_string()).bold(),
                    "Start scanning",
                ),
                (
                    Cell::from({
                        if config.toggle_connect == ' ' {
                            "Space".to_string()
                        } else {
                            config.toggle_connect.to_string()
                        }
                    })
                    .bold(),
                    "Connect/Disconnect the network",
                ),
                (Cell::from("?").bold(), "Show help"),
                (Cell::from("q or ctrl+c").bold(), "Quit"),
                (Cell::from(""), ""),
                (
                    Cell::from("## Device").style(Style::new().bold().fg(Color::Yellow)),
                    "",
                ),
                (
                    Cell::from(config.device.infos.to_string()).bold(),
                    "Show device information",
                ),
                (Cell::from(""), ""),
                (
                    Cell::from("## Known Networks").style(Style::new().bold().fg(Color::Yellow)),
                    "",
                ),
                (
                    Cell::from(config.known_network.remove.to_string()).bold(),
                    "Remove the network from the known networks list",
                ),
            ],
        }
    }

    pub fn scroll_down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.keys.len().saturating_sub(self.block_height - 6) {
                    i
                } else {
                    i + 1
                }
            }
            None => 1,
        };
        *self.state.offset_mut() = i;
        self.state.select(Some(i));
    }
    pub fn scroll_up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 1 {
                    i - 1
                } else {
                    0
                }
            }
            None => 1,
        };
        *self.state.offset_mut() = i;
        self.state.select(Some(i));
    }

    pub fn render(&mut self, frame: &mut Frame, color_mode: ColorMode) {
        let block = help_rect(frame.size());

        self.block_height = block.height as usize;
        let widths = [Constraint::Length(20), Constraint::Fill(1)];
        let rows: Vec<Row> = self
            .keys
            .iter()
            .map(|key| {
                Row::new(vec![key.0.to_owned(), key.1.into()]).style(match color_mode {
                    ColorMode::Dark => Style::default().fg(Color::White),
                    ColorMode::Light => Style::default().fg(Color::Black),
                })
            })
            .collect();
        let rows_len = self.keys.len().saturating_sub(self.block_height - 6);

        let table = Table::new(rows, widths).block(
            Block::default()
                .padding(Padding::uniform(2))
                .title(" Help ")
                .title_style(Style::default().bold().fg(Color::Green))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(Clear, block);
        frame.render_stateful_widget(table, block, &mut self.state);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state =
            ScrollbarState::new(rows_len).position(self.state.selected().unwrap_or_default());
        frame.render_stateful_widget(
            scrollbar,
            block.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

pub fn help_rect(r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Min(10),
                Constraint::Percentage(35),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length((r.width - 80) / 2),
                Constraint::Min(80),
                Constraint::Length((r.width - 80) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
