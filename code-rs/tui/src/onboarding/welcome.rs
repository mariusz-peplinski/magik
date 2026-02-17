use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::WidgetRef;

use crate::onboarding::onboarding_screen::StepStateProvider;

use super::onboarding_screen::StepState;

pub(crate) struct WelcomeWidget {
    pub is_logged_in: bool,
}

impl WidgetRef for &WelcomeWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let version = format!("v{}", code_version::version());
        let title = format!("Magik Code {version}");
        let line1 = Line::from(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ));
        line1.render(area, buf);

        if area.height <= 1 {
            return;
        }

        let line2 = Line::from(vec![
            Span::styled(
                "by @mariusz-peplinski ",
                Style::default().fg(crate::colors::text_dim()),
            ),
            Span::styled(
                "<3",
                Style::default()
                    .fg(ratatui::style::Color::Rgb(255, 20, 147))
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        let line2_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };
        line2.render(line2_area, buf);
    }
}

impl StepStateProvider for WelcomeWidget {
    fn get_step_state(&self) -> StepState {
        match self.is_logged_in {
            true => StepState::Hidden,
            false => StepState::Complete,
        }
    }
}
