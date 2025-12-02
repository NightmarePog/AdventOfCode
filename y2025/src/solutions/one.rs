use crate::utils::input_http;

pub async fn init() {
    let input = input_http::get(1).await.unwrap();
    process(input);
}

struct Dial {
    x: i32,
}

impl Dial {
    fn new() -> Self {
        Dial { x: 50 }
    }

    fn get_position(&self) -> i32 {
        self.x
    }

    fn left(&mut self, count: u16) -> i32 {
        let mut click = 0;
        for _ in 0..count {
            self.x = (self.x - 1).rem_euclid(100);
            if self.x == 0 {
                click += 1;
            }
        }
        click
    }

    fn right(&mut self, count: u16) -> i32 {
        let mut click = 0;
        for _ in 0..count {
            self.x = (self.x + 1) % 100;
            if self.x == 0 {
                click += 1;
            }
        }
        click
    }
}

fn process(moves: Vec<String>) {
    let mut dial = Dial::new();
    let mut zero_pass_count = 0;

    println!("The dial starts by pointing at 50.");

    for (i, m) in moves.iter().enumerate() {
        println!("{}", i);

        let direction = m.chars().next().expect("empty line");
        let count: u16 = m[1..].parse().expect("invalid format");

        match direction {
            'L' => zero_pass_count += dial.left(count),
            'R' => zero_pass_count += dial.right(count),
            _ => panic!("Invalid direction"),
        }

        println!(
            "The dial is rotated {} to point at {}",
            m,
            dial.get_position()
        );
    }

    println!("to zero it got {}", zero_pass_count);
}
