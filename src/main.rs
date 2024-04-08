use rand::{thread_rng, Rng};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{self, Time, Vector2f, Vector2u},
    window::{Key, Style},
};

const SPEED: f32 = 400f32;
const SPEED_FACTOR: f32 = 2f32;
const HEAD_COLOR: Color = Color::BLUE;
const TAIL_COLOR: Color = Color::GREEN;
const FOOD_COLOR: Color = Color::RED;
const FOOD_SIZE: f32 = 20f32;
const SEG_SIZE: f32 = 40f32;
const STARTING_POS: (f32, f32) = (1366f32 / 2.0, 768f32 / 2.0);

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

struct Snake<'a> {
    head: RectangleShape<'a>,
    tail: Vec<RectangleShape<'a>>,
    direction: Direction,
    current_speed: f32,
}
impl<'a> Snake<'a> {
    fn build_seg(last_tail_pos: Vector2f) -> RectangleShape<'a> {
        let mut seg = RectangleShape::new();
        seg.set_position(last_tail_pos);
        seg.set_origin((SEG_SIZE / 2.0, SEG_SIZE / 2.0));
        seg.set_fill_color(TAIL_COLOR);
        seg.set_size((SEG_SIZE, SEG_SIZE));
        seg
    }
    fn new() -> Self {
        let mut head = RectangleShape::new();
        head.set_position(STARTING_POS);
        head.set_origin((SEG_SIZE / 2.0, SEG_SIZE / 2.0));
        head.set_fill_color(HEAD_COLOR);
        head.set_size((SEG_SIZE, SEG_SIZE));
        let mut tail = vec![];
        tail.push(Self::build_seg(head.position()));
        Self {
            head,
            tail,
            direction: Direction::Left,
            current_speed: SPEED,
        }
    }
    fn move_(&mut self, dt: Time) {
        let (mut x, mut y): (f32, f32);
        match self.direction {
            Direction::Up => {
                x = 0.0;
                y = -self.current_speed;
            }
            Direction::Left => {
                x = -self.current_speed;
                y = 0.0
            }
            Direction::Right => {
                x = self.current_speed;
                y = 0.0;
            }
            Direction::Down => {
                x = 0.0;
                y = self.current_speed;
            }
        }
        for i in (1..self.tail.len()).rev() {
            let next_pos = self.tail[i - 1].position();
            self.tail[i].set_position(next_pos);
        }
        x *= dt.as_seconds();
        y *= dt.as_seconds();
        self.tail
            .first_mut()
            .unwrap()
            .set_position(self.head.position());
        self.head.move_((x, y));
    }
    fn draw(&mut self, window: &mut RenderWindow) {
        window.draw(&self.head);
        for seg in self.tail.iter() {
            window.draw(seg);
        }
    }
    fn intersect(&self, obj: &impl Shape<'a>) -> bool {
        self.head
            .global_bounds()
            .intersection(&obj.global_bounds())
            .is_some()
    }
    fn grow(&mut self) {
        for _ in 0..8 {
            let back_pos = self.tail.last().unwrap().position();
            self.tail.push(Self::build_seg(back_pos));
        }
    }
    fn position(&self) -> Vector2f {
        self.head.position()
    }
}
fn main() {
    //initlize a window
    let mut window =
        RenderWindow::new((1366, 768), "Snake Game", Style::CLOSE, &Default::default());

    //init the snake
    let mut snake = Snake::new();
    let mut food = RectangleShape::new();
    food.set_fill_color(FOOD_COLOR);
    food.set_size((FOOD_SIZE, FOOD_SIZE));
    let get_food_pos = || {
        (
            thread_rng().gen_range(100..1000) as f32,
            thread_rng().gen_range(100..600) as f32,
        )
    };
    food.set_position(get_food_pos());
    let mut clock = system::Clock::start();
    window.set_framerate_limit(60);
    let mut shift_pressed = false;
    while window.is_open() {
        //handle the input
        if let Some(event) = window.poll_event() {
            match event {
                sfml::window::Event::KeyPressed { code, shift, .. } => {
                    shift_pressed = shift;
                    match code {
                        Key::Escape => window.close(),
                        Key::J => {
                            if snake.direction != Direction::Up {
                                snake.direction = Direction::Down
                            }
                        }
                        Key::L => {
                            if snake.direction != Direction::Left {
                                snake.direction = Direction::Right
                            }
                        }
                        Key::H => {
                            if snake.direction != Direction::Right {
                                snake.direction = Direction::Left
                            }
                        }
                        Key::K => {
                            if snake.direction != Direction::Down {
                                snake.direction = Direction::Up
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        window.clear(Color::BLACK);
        let dt = clock.restart();
        if shift_pressed {
            snake.current_speed *= SPEED_FACTOR;
        }
        snake.move_(dt);

        //handle the crash with the boundaries
        let Vector2u {
            x: window_width,
            y: window_height,
        } = window.size();
        let Vector2f {
            x: snake_x,
            y: snake_y,
        } = snake.position();
        if snake_x < 0.0
            || snake_x > window_width as f32
            || snake_y < 0.0
            || snake_y > window_height as f32
        {
            println!("You crashed with the boundaries");
            window.close();
            break;
        }

        //handle the crashe with the snake itself
        for seg in snake.tail.iter().skip(16).rev() {
            if snake.intersect(seg) {
                println!("You crashed with yourself");
                window.close();
                break;
            }
        }
        snake.current_speed = SPEED;
        if snake.intersect(&food) {
            food.set_position(get_food_pos());
            snake.grow();
        }

        snake.draw(&mut window);
        window.draw(&food);
        window.display();
    }
}
