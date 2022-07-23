use std::f32::consts::PI;

use ggez::*;
use glam::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
struct Object {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    radius: f32,
    color: graphics::Color,
}

impl Object {
    fn new(ctx: &Context) -> Object {
        let mut rng = rand::thread_rng();
        let distance: f32 = rng.gen_range(100.0..1500.0);
        let angle: f32 = rng.gen_range(0.0..2.0 * PI);
        let velocity: f32 = rng.gen_range(0.0..1.0);
        let mass: f32 = rng.gen_range(10.0..15.0);
        let radius: f32 = mass.sqrt() * 2.0;
        Object {
            position: vec2(distance * angle.sin(), distance * angle.cos()),
            velocity: vec2(velocity * (angle+PI/2.0).sin(), velocity * (angle+PI/2.0).cos()),
            acceleration: vec2(0.0, 0.0),
            mass,
            radius,
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
    fn isColliding(&self, other: &Object) -> bool {
         self.position.distance(other.position) <= (self.radius + other.radius) 
    }
    fn merge(&mut self, other: &Object) {
        self.position = ( self.position*self.mass + other.position*other.mass) / (self.mass+other.mass);
        self.velocity = ( self.velocity*self.mass + other.velocity*other.mass) / (self.mass+other.mass);
        self.mass = self.mass+other.mass;
        self.radius = self.mass.sqrt() * 2.0;
    }
    fn calculate_force(&mut self, other: &Object) {
        self.acceleration = vec2(0.0, 0.0);
        //let G = 6.674e-11;
        let G = 1.0;
        let r_squared = self.position.distance_squared(other.position);
        let normal = -(self.position - other.position).normalize();
        self.acceleration +=
            ((((G * (self.mass as f64) * (other.mass as f64)) / r_squared as f64) as f32) * normal)
                / self.mass;
    }
    fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
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
        for _ in 0..rng.gen_range(500..1000) {
            objects.push(Object::new(&ctx));
        }
        objects.push(Object::new(&ctx).with_position(vec2(0.0,0.0)).with_velocity(vec2(0.0,0.0)).with_mass(1000.0).with_radius(1000.0f32.sqrt()*2.0));
        Ok(State {
            dt: ctx.time.delta(),
            objects: objects,
        })
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        //Determine forces
        for i in 0..self.objects.len() {
            for j in 0..self.objects.len() {
                if i != j {
                    let other = self.objects[j].clone();
                    self.objects[i].calculate_force(&other);
                }
            }
        }
        //Update velocity/position
        for i in 0..self.objects.len() {
            self.objects[i].update(10.0);
        }
        //Check for collisions
        let mut i = 0;
        while i < self.objects.len()-1 {
            let mut j = i+1;
            while j < self.objects.len() {
                if self.objects[i].isColliding(&self.objects[j]) {
                    let other = self.objects[j].clone();
                    self.objects[i].merge(&other);
                    self.objects.remove(j);
                    j = i+1;
                } else {j+=1;}
            }
            i += 1
        }
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
        let full_x = (max_x - min_x) * 1.2;
        let full_y = (max_y - min_x) * 1.2;

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

        //println!(
        //    "dt = {}ns  {}x{} {:?}",
        //    self.dt.as_nanos(),
        //    width,
        //    height,
        //    self.objects[0],
        //);

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
