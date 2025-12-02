use crate::utils::input_http;

pub async fn init() {
    let input = input_http::get(1).await.unwrap();
    process(input);
}

struct Dial {
    x: u16,
}

impl Dial {
    fn new() -> Self {
        Dial { x: 50 }
    }

    fn left(&mut self, count: u16) {
        let new_x = (self.x as i16 - count as i16).rem_euclid(100);
        self.x = new_x as u16;
    }

    fn right(&mut self, count: u16) {
        self.x = (self.x + count) % 100;
    }
}

fn process(moves: Vec<String>) {
    let mut dial = Dial::new();
    let mut zero_count = 0;
    for m in moves {
        if dial.x == 0 {
            zero_count += 1;
        }
        let direction: char = m.chars().next().unwrap();
        let count: u16 = m.chars().skip(1).collect::<String>().parse().unwrap();

        match direction {
            'L' => dial.left(count),
            'R' => dial.right(count),
            _ => panic!("Invalid direction"),
        }
        //println!("final direction: {}", dial.x);
    }
    println!("final direction: {}", zero_count);
}
