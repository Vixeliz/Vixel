use std::{env, path};

use ggegui::{egui, Gui};
use ggez::context::HasMut;
use ggez::glam::{UVec2, Vec2};
use ggez::graphics::{GraphicsContext, Image, ImageFormat, Sampler};
use ggez::input::keyboard::KeyCode;
use ggez::GameError;
use ggez::{
    event::{self, EventHandler},
    glam,
    graphics::{self, Color, DrawParam},
    Context, ContextBuilder, GameResult,
};

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Vixel", "Vixeliz")
        .window_mode(ggez::conf::WindowMode {
            resizable: true,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
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

struct PixCanvas {
    cpu: Vec<Color>,
    canvas: Image,
    size: UVec2,
}

impl PixCanvas {
    fn new(gfx: &mut impl HasMut<GraphicsContext>, size: UVec2) -> Self {
        let gfx = gfx.retrieve_mut();
        let canvas = Image::new_canvas_image(gfx, ImageFormat::Bgra8UnormSrgb, size.x, size.y, 1);
        let cpu = vec![Color::BLACK; size.x as usize * size.y as usize];
        Self { cpu, canvas, size }
    }

    fn linearize(&self, pos: UVec2) -> usize {
        ((pos.x * self.size.x) + pos.y) as usize
    }

    fn set_pixel(&mut self, pos: UVec2, color: Color) -> GameResult {
        let idx = self.linearize(pos);
        let pixel = self
            .cpu
            .get_mut(idx)
            .ok_or(GameError::CustomError("Pixel out of bounds".to_string()))?;
        *pixel = color;
        Ok(())
    }
}

struct State {
    gui: Gui,
    command: String,
    mode: Mode,
    center: Vec2,
    pixel_canvas: PixCanvas,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let mut gui = Gui::new(ctx);
        gui.input.set_scale_factor(1.75, ctx.gfx.size());
        let pixel_canvas = PixCanvas::new(ctx, UVec2::splat(16));
        Self {
            gui,
            command: "".to_string(),
            mode: Mode::Edit,
            center: Vec2::ZERO,
            pixel_canvas,
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
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            self.pixel_canvas.set_pixel(UVec2::ZERO, Color::GREEN)?;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::V) {
            if self.mode == Mode::Edit {
                self.mode = Mode::Visual;
            } else if self.mode == Mode::Visual {
                self.mode = Mode::Edit;
            }
        }

        let gui_ctx = self.gui.ctx();

        let center = egui::CentralPanel::default()
            .show(&gui_ctx, |ui| {
                egui::warn_if_debug_build(ui);
            })
            .response
            .rect
            .center();

        self.center = Vec2::new(center.x * 1.75, center.y * 1.75);

        egui::TopBottomPanel::bottom("status_bar").show(&gui_ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label(self.mode.get_str());
                    ui.label("|");
                    ui.label("resources/hacker.jpg");
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
        let canvas =
            graphics::Canvas::from_image(ctx, self.pixel_canvas.canvas.clone(), Color::BLACK);
        canvas.finish(ctx)?;
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.draw(
            &self.pixel_canvas.canvas,
            DrawParam::default()
                .offset(Vec2::new(0.5, 0.5))
                .dest(self.center)
                .scale(Vec2::splat(20.0)),
        );
        canvas.finish(ctx)?;

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) -> GameResult {
        self.gui.input.text_input_event(character);
        Ok(())
    }
}
