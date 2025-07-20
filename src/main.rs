use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 6;

fn main() {
    let mut buffer = vec![0u32; (WIDTH * SCALE) * (HEIGHT * SCALE)];
    let mut world = vec![vec![false; WIDTH]; HEIGHT];

    // === Agregar patrones aquí ===
    insert_pattern(&mut world, 1, 1, &glider());
    insert_pattern(&mut world, 40, 30, &pulsar());
    insert_pattern(&mut world, 70, 20, &lwss());

    // === Crear ventana ===
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    ).unwrap();

    // === Loop principal ===
    while window.is_open() && !window.is_key_down(Key::Escape) {
        world = next_gen(&world);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if world[y][x] { 0xFFFFFF } else { 0x000000 };
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let px = x * SCALE + dx;
                        let py = y * SCALE + dy;
                        buffer[py * WIDTH * SCALE + px] = color;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

// === Lógica de Conway ===
fn next_gen(world: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_world = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut neighbors = 0;

            for dy in [-1i32, 0, 1] {
                for dx in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = ((x as i32 + dx + WIDTH as i32) % WIDTH as i32) as usize;
                    let ny = ((y as i32 + dy + HEIGHT as i32) % HEIGHT as i32) as usize;

                    if world[ny][nx] {
                        neighbors += 1;
                    }
                }
            }

            new_world[y][x] = match (world[y][x], neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_world
}

// === Insertar patrón en el mundo ===
fn insert_pattern(world: &mut Vec<Vec<bool>>, x: usize, y: usize, pattern: &[(usize, usize)]) {
    for &(dx, dy) in pattern {
        let nx = x + dx;
        let ny = y + dy;
        if ny < world.len() && nx < world[0].len() {
            world[ny][nx] = true;
        }
    }
}

// === Patrones ===
fn glider() -> Vec<(usize, usize)> {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}

fn pulsar() -> Vec<(usize, usize)> {
    vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0,10), (5,10), (7,10), (12,10),
        (2,12), (3,12), (4,12), (8,12), (9,12), (10,12),
    ]
}

fn lwss() -> Vec<(usize, usize)> {
    vec![
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 3)
    ]
}