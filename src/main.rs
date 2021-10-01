use rand::Rng; // 0.8.0

const HEIGHT: usize = 60;
const WIDTH: usize = 30;
const POONUMBER: i32 = 30;

const SYMBOLS: Symbols = Symbols {
    snake: '@',
    wall: '#',
    poo: '¤',
    space: ' ',
};

struct Symbols {
    snake: char,
    wall: char,
    poo: char,
    space: char,
}

struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    symbol: char,
    position: Point,
}

fn generate_map() -> [[i32; HEIGHT]; WIDTH] {
    let mut poo_counter: i32 = 0;
    let mut tmp_map = [[0i32; HEIGHT]; WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if x == 0 || x == WIDTH - 1 {
                if y == HEIGHT - 1 {
                    tmp_map[x][y] = 1;
                } else {
                    tmp_map[x][y] = 1;
                }
            } else {
                if y == 0 {
                    tmp_map[x][y] = 1;
                } else {
                    if y == HEIGHT - 1 {
                        tmp_map[x][y] = 1;
                    } else {
                        let num = rand::thread_rng().gen_range(0..HEIGHT);
                        let modulo = rand::thread_rng().gen_range(1..HEIGHT * WIDTH);
                        if num % modulo == 0 && poo_counter < POONUMBER {
                            tmp_map[x][y] = 2;
                            poo_counter += 1;
                        } else {
                            tmp_map[x][y] = 0;
                        }
                    }
                }
            }
        }
    }
    return tmp_map;
}

fn print_section(symbol: char, y: usize) {
    if y == HEIGHT - 1 {
        println!("{}", symbol);
    } else {
        print!("{}", symbol);
    }
}

fn draw_map(map: [[i32; HEIGHT]; WIDTH], snake: Snake) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if (x as i32) == snake.position.x && (y as i32) == snake.position.y {
                print!("{}", snake.symbol);
            } else {
                match map[x][y] {
                    0 => print_section(SYMBOLS.space, y),
                    1 => print_section(SYMBOLS.wall, y),
                    2 => print_section(SYMBOLS.poo, y),
                    _ => {
                        print!("");
                    }
                }
            }
        }
    }
}

fn main() {
    let map = generate_map();
    let snake: Snake = Snake {
        symbol: SYMBOLS.snake,
        position: Point {
            x: rand::thread_rng().gen_range(1..(HEIGHT - 1) as i32),
            y: rand::thread_rng().gen_range(1..(WIDTH - 1) as i32),
        },
    };

    println!(
        "Snake symbol: {}, place: x: {}, y: {}",
        snake.symbol, snake.position.x, snake.position.y
    );
    draw_map(map, snake);
}
