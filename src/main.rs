use ggegui::{egui, Gui};
use ggez::input::keyboard::KeyCode;
use ggez::{
    event::{self, EventHandler},
    glam,
    graphics::{self, Color, DrawParam},
    Context, ContextBuilder, GameResult,
};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Vixel", "Vixeliz")
        .window_mode(ggez::conf::WindowMode {
            resizable: true,
            ..Default::default()
        })
        .build()
        .unwrap();
    let state = State::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Mode {
    Command,
    Visual,
    Edit,
}

impl Mode {
    fn get_str(&self) -> String {
        match self {
            Self::Command => "CMD".to_owned(),
            Self::Edit => "EDIT".to_owned(),
            Self::Visual => "VIS".to_owned(),
        }
    }
}

struct State {
    gui: Gui,
    command: String,
    mode: Mode,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let mut gui = Gui::new(ctx);
        gui.input.set_scale_factor(1.75, ctx.gfx.size());
        Self {
            gui,
            command: "".to_string(),
            mode: Mode::Edit,
        }
    }

    fn process_command(&mut self, _: &mut Context) {
        self.mode = Mode::Edit;
        self.command.clear();
    }
}

impl EventHandler for State {
    fn resize_event(&mut self, _: &mut Context, width: f32, height: f32) -> GameResult {
        self.gui.input.resize_event(width, height);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Colon) {
            self.mode = Mode::Command;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::V) {
            if self.mode == Mode::Edit {
                self.mode = Mode::Visual;
            } else if self.mode == Mode::Visual {
                self.mode = Mode::Edit;
            }
        }

        let gui_ctx = self.gui.ctx();

        egui::CentralPanel::default().show(&gui_ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });

        egui::TopBottomPanel::bottom("status_bar").show(&gui_ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label(self.mode.get_str());
                    ui.label("|");
                    ui.label("assets/hacker.jpg");
                });
                ui.separator();
                if self.mode == Mode::Command {
                    let res = ui.add(egui::TextEdit::singleline(&mut self.command).hint_text(""));
                    if res.lost_focus() {
                        self.process_command(ctx);
                    }
                    res.request_focus();
                } else {
                    ui.add(egui::TextEdit::singleline(&mut self.command).hint_text(""));
                }
            });
        });
        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) -> GameResult {
        self.gui.input.text_input_event(character);
        Ok(())
    }
}
