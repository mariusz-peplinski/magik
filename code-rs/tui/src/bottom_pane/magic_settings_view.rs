use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::colors;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

pub(crate) struct MagicSettingsView {
    app_event_tx: AppEventSender,
    selected_index: usize,
    show_reasoning: bool,
    show_block_type_labels: bool,
    rounded_corners: bool,
    is_complete: bool,
}

impl MagicSettingsView {
    pub(crate) fn new(
        app_event_tx: AppEventSender,
        show_reasoning: bool,
        show_block_type_labels: bool,
        rounded_corners: bool,
    ) -> Self {
        Self {
            app_event_tx,
            selected_index: 0,
            show_reasoning,
            show_block_type_labels,
            rounded_corners,
            is_complete: false,
        }
    }

    fn option_count() -> usize {
        4
    }

    fn close(&mut self) {
        self.is_complete = true;
    }

    fn toggle_show_reasoning(&mut self) {
        self.show_reasoning = !self.show_reasoning;
        self.app_event_tx
            .send(AppEvent::SetTuiShowReasoning(self.show_reasoning));
    }

    fn toggle_show_block_type_labels(&mut self) {
        self.show_block_type_labels = !self.show_block_type_labels;
        self.app_event_tx
            .send(AppEvent::SetTuiShowBlockTypeLabels(self.show_block_type_labels));
    }

    fn toggle_rounded_corners(&mut self) {
        self.rounded_corners = !self.rounded_corners;
        self.app_event_tx
            .send(AppEvent::SetTuiRoundedCorners(self.rounded_corners));
    }

    fn activate_selected(&mut self) {
        match self.selected_index {
            0 => self.toggle_show_reasoning(),
            1 => self.toggle_show_block_type_labels(),
            2 => self.toggle_rounded_corners(),
            3 => self.close(),
            _ => {}
        }
    }

    fn info_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(Line::from(vec![Span::styled(
            "Magik Settings",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        let highlight = Style::default()
            .fg(colors::primary())
            .add_modifier(Modifier::BOLD);
        let normal = Style::default().fg(colors::text());
        let dim = Style::default().fg(colors::text_dim());

        let row_toggle = |idx: usize, label: &str, enabled: bool| -> Line<'static> {
            let selected = idx == self.selected_index;
            let indicator = if selected { ">" } else { " " };
            let style = if selected { highlight } else { normal };
            let state_style = if enabled {
                Style::default().fg(colors::success())
            } else {
                Style::default().fg(colors::text_dim())
            };

            Line::from(vec![
                Span::styled(format!("{indicator} "), style),
                Span::styled(label.to_string(), style),
                Span::raw("  "),
                Span::styled(
                    format!("[{}]", if enabled { "x" } else { " " }),
                    state_style,
                ),
            ])
        };

        lines.push(row_toggle(0, "Show reasoning", self.show_reasoning));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "Always display model reasoning blocks when present.",
                dim,
            ),
        ]));

        lines.push(row_toggle(
            1,
            "Show block type labels",
            self.show_block_type_labels,
        ));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "Adds a colored [TYPE] header above displayed blocks.",
                dim,
            ),
        ]));

        lines.push(row_toggle(2, "Rounded corners", self.rounded_corners));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "Render bordered boxes with rounded corners (╭╮╯╰).",
                dim,
            ),
        ]));

        lines.push(Line::from(""));

        let close_selected = self.selected_index == 3;
        let close_style = if close_selected { highlight } else { normal };
        let indicator = if close_selected { ">" } else { " " };
        lines.push(Line::from(vec![
            Span::styled(format!("{indicator} "), close_style),
            Span::styled("Close", close_style),
        ]));

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(" Up/Down", Style::default().fg(colors::function())),
            Span::styled(" Navigate  ", dim),
            Span::styled("Enter", Style::default().fg(colors::success())),
            Span::styled(" Toggle  ", dim),
            Span::styled("Esc", Style::default().fg(colors::error())),
            Span::styled(" Close", dim),
        ]));

        lines
    }

    pub(crate) fn render_without_frame(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        Paragraph::new(self.info_lines())
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(colors::background()).fg(colors::text()))
            .render(area, buf);
    }

    pub(crate) fn handle_key_event_direct(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.close(),
            KeyCode::Up => {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Tab => {
                self.selected_index = (self.selected_index + 1) % Self::option_count();
            }
            KeyCode::BackTab => {
                if self.selected_index == 0 {
                    self.selected_index = Self::option_count() - 1;
                } else {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
            KeyCode::Enter => self.activate_selected(),
            _ => {}
        }
    }

    pub(crate) fn is_view_complete(&self) -> bool {
        self.is_complete
    }
}
