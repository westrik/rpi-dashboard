use chrono::Local;
use druid::widget::prelude::*;
use druid::widget::{Controller, CrossAxisAlignment, Flex, Label};
use druid::{AppLauncher, Data, Lens, LocalizedString, TimerToken, WidgetExt, WindowDesc};
use std::time::Duration;

#[derive(Clone, Debug, Data, Lens)]
struct AppState {
    current_time: Option<String>,
}

impl AppState {
    pub fn iter_interval(&self) -> u64 {
        (1000. / self.fps()) as u64
    }
    pub fn fps(&self) -> f64 {
        60.0
    }
}

struct ClockController {
    timer_id: TimerToken,
}

impl Controller<AppState, Label<AppState>> for ClockController {
    fn event(
        &mut self,
        _: &mut Label<AppState>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        _: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_millis(data.iter_interval());
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    data.current_time = Some(Local::now().format("%F %X").to_string());
                    ctx.request_layout();
                    let deadline = Duration::from_millis(data.iter_interval());
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            _ => {}
        }
    }
}

fn make_widget() -> impl Widget<AppState> {
    let clock_label = Label::new(|s: &AppState, _: &Env| {
        s.current_time
            .clone()
            .unwrap_or_else(|| "No time set".to_string())
    })
    .with_text_size(32.)
    .controller(ClockController {
        timer_id: TimerToken::INVALID,
    });
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(clock_label)
        .center()
}

pub fn main() {
    let window = WindowDesc::new(make_widget)
        .window_size(Size {
            width: 720.0,
            height: 720.0,
        })
        .resizable(false)
        .title(LocalizedString::new("rpi-dashboard").with_placeholder("RPi Dashboard"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState { current_time: None })
        .expect("launch failed");
}
