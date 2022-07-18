use std::f32::consts::PI;

use ggez::*;
use glam::*;
use rand::Rng;

#[derive(Clone, Copy)]
struct Object {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: graphics::Color,
}

impl Object {
    fn new(ctx: &Context) -> Object {
        let mut rng = rand::thread_rng();
        let (width, height) = ctx.gfx.drawable_size();
        let mut max_distance = width / 2.0;
        if height > width {
            max_distance = height / 2.0;
        }
        let distance = rng.gen_range(10.0..max_distance);
        let angle = rng.gen_range(0.0..2.0 * PI);
        Object {
            position: vec2(distance * angle.sin(), distance * angle.cos()),
            velocity: vec2(rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0)),
            mass: rng.gen_range(10.0..100.0),
            radius: rng.gen_range(0.2..10.0),
            color: graphics::Color::WHITE,
        }
    }
    fn with_position(mut self, position: Vec2) -> Object {
        self.position = position;
        self
    }
    fn with_velocity(mut self, velocity: Vec2) -> Object {
        self.velocity = velocity;
        self
    }
    fn with_mass(mut self, mass: f32) -> Object {
        self.mass = mass;
        self
    }
    fn with_radius(mut self, radius: f32) -> Object {
        self.radius = radius;
        self
    }
    fn with_color(mut self, color: graphics::Color) -> Object {
        self.color = color;
        self
    }
    fn draw(self, ctx: &Context, offset: Vec2) -> GameResult<graphics::Mesh> {
        graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position + offset,
            self.radius,
            2.0,
            graphics::Color::WHITE,
        )
    }
}

struct State {
    dt: std::time::Duration,
    objects: Vec<Object>,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let mut rng = rand::thread_rng();
        let mut objects: Vec<Object> = vec![];
        for _ in 0..rng.gen_range(10..100) {
            objects.push(Object::new(&ctx));
        }
        Ok(State {
            dt: ctx.time.delta(),
            objects: objects,
        })
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (width, height) = ctx.gfx.drawable_size();
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        //Get min/max dimensions
        let mut min_x = self.objects[0].position.x;
        let mut max_x = self.objects[0].position.x;
        let mut min_y = self.objects[0].position.y;
        let mut max_y = self.objects[0].position.y;
        for object in &self.objects {
            if object.position.x < min_x {
                min_x = object.position.x;
            }
            if object.position.x > max_x {
                max_x = object.position.x;
            }
            if object.position.y < min_y {
                min_y = object.position.y;
            }
            if object.position.y > max_y {
                max_y = object.position.y;
            }
        }

        //Space used
        let full_x = max_x - min_x;
        let full_y = max_y - min_x;

        //Determine the scaling needed
        let scale_x = width / full_x;
        let scale_y = height / full_y;
        let scale = if scale_y < scale_x { scale_y } else { scale_x };

        //Generate offset for greater scale
        //Adjust by range of objects to align to edge, and then by scaled distance to center
        let width_scaled = (max_x - min_x) * scale;
        let height_scaled = (max_y - min_y) * scale;
        let width_offset_scaled = (width - width_scaled) / 2.0;
        let height_offset_scaled = (height - height_scaled) / 2.0;
        let width_offset = width_offset_scaled / scale;
        let height_offset = height_offset_scaled / scale;
        let offset = vec2(-min_x + width_offset, -min_y + height_offset);

        println!(
            "dt = {}ns  {}x{}",
            self.dt.as_nanos(),
            width,
            height,
        );

        for object in &self.objects {
            canvas.draw(
                &object.draw(&ctx, offset).unwrap(),
                graphics::DrawParam::default().scale(vec2(scale, scale)),
            );
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let mut c = conf::Conf::new();
    c.window_mode.resizable = true;
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    let state = State::new(&mut ctx)?;
    println!("Hello, world!");
    event::run(ctx, event_loop, state);
}
