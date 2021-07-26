use miniquad::*;
use macroquad::prelude::*;
use macroquad::window::InternalGlContext;
// use nona::widgets::{Widget, Button};
use nonaquad::nvgimpl as nvgimpl;
use nona::{Point, Paint};
use std::time::Instant;

struct Stage<'a> {
    renderer: Option<nvgimpl::Renderer<'a>>,
    nona: nona::Context<nvgimpl::Renderer<'a>>,
}

impl<'a> Stage<'a> {
    pub fn new(ctx: &'a mut Context) -> Stage<'a> {
        let mut renderer = nvgimpl::Renderer::create(ctx).unwrap();
        let nona = nona::Context::create(&mut renderer).unwrap();
        Stage {
            renderer: Some(renderer),
            nona,
        }
    }
}

impl<'a> EventHandlerFree for Stage<'a> {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // let ctx = get_context();

        let nona = &mut self.nona;
        nona.attach_renderer(self.renderer.take());

        nona.begin_frame(None).unwrap();


        nona.end_frame().unwrap();

        self.renderer = nona.detach_renderer();
    }
}

struct Rustaceane {
    pos: Vec2,
    speed: Vec2,
    color: Color
}



#[macroquad::main("vector_render")]
async fn main() {

    let mut rustaceanes: Vec<Rustaceane> = Vec::new();

    let mut stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        Stage::new(ctx)
    };

    loop {

        if macroquad::input::is_mouse_button_down(MouseButton::Left) {
            for _i in 0..100 {
                rustaceanes.push(Rustaceane {
                    pos: Vec2::from(macroquad::input::mouse_position()),
                    speed: Vec2::new(rand::gen_range(-250., 250.) / 60.,
                                     rand::gen_range(-250., 250.) / 60.),
                    color: Color::from_rgba(rand::gen_range(50, 240),
                                            rand::gen_range(80, 240),
                                            rand::gen_range(100, 240),
                                            255)
                })
            }
        }

        clear_background(BLACK);

        stage.nona.attach_renderer(stage.renderer.take());

        stage.nona.begin_frame(None).unwrap();

        for rustaceane in &mut rustaceanes {
            rustaceane.pos += rustaceane.speed;

            if ((rustaceane.pos.x + 32. / 2.) > screen_width()) ||
                ((rustaceane.pos.x + 32. / 2.) < 0.) {
                rustaceane.speed.x *= -1.;
            }
            if ((rustaceane.pos.y + 32. / 2.) > screen_height()) ||
                ((rustaceane.pos.y + 32. / 2.) < 0.) {
                rustaceane.speed.y *= -1.;
            }

            stage.nona.begin_path();
            stage.nona.circle(Point::new(rustaceane.pos.x, rustaceane.pos.y), 32.);
            stage.nona.fill_paint(Paint::from(nona::Color::rgb_i(255, 0,0)));
            stage.nona.fill().unwrap();
        }

        stage.nona.end_frame().unwrap();
        stage.renderer = stage.nona.detach_renderer();

        draw_text(format!("FPS: {}",get_fps()).as_str(), 0., 16., 32., WHITE);
        draw_text(format!("Rustaceanes: {}", rustaceanes.len()).as_str(), 0., 32., 32., WHITE);

        next_frame().await
    }

}
