use colored::Colorize;
use termsize::Size;

struct Field {
    name: String,
    size: Size,
    objects: Option<Vec<Object>>,
}

impl Field {
    fn new(name: String, objects: Option<Vec<Object>>) -> Self {
        let terminal_size = termsize::get().unwrap();
        Field {
            name,
            size: terminal_size, // Rewrite it to tuple later. It will allow more versatile approaches.
            objects,
        }
    }

    fn render(&self) {
        for _ in 0..self.size.rows {
            for _ in 0..self.size.cols {
                print!("{}", "&".bright_black());
            }
            println!();
        }
    }

    fn resize(&mut self) {
        let terminal_size = termsize::get().unwrap();
        self.size = terminal_size;
    }
}

struct Object {
    name: String,
}

struct AnimatedObject {
    name: String,
    animation: Vec<Sprite>,
}

struct Sprite {
    image: Vec<&'static [char]>,
}

trait Drawable {
    fn draw(&self) -> Sprite;
}

fn main() {
    println!("Hello, world!");
    let a = Field::new("main_field".to_string(), None);

    loop {
        a.render();
    }
}
