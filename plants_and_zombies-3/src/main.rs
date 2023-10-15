use std::fmt::{Debug, Formatter};
use std::thread;
use std::time::Duration;

use itertools::Itertools;

macro_rules! sleep {
    ($x:expr) => {
        thread::sleep(Duration::from_millis($x));
    };
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn is_diagonal_ne(&self, other: &Point) -> bool {
        let Point(x, y) = self;
        let Point(x1, y1) = other;
        let dx = x1.max(x) - x1.min(x);
        let dy = y1.max(y) - y1.min(y);
        dx == dy && y1 < y
    }

    fn is_diagonal_se(&self, other: &Point) -> bool {
        let Point(x, y) = self;
        let Point(x1, y1) = other;
        let dx = x1.max(x) - x1.min(x);
        let dy = y1.max(y) - y1.min(y);
        dx == dy && y1 > y
    }

    fn is_horizontal(&self, other: &Point) -> bool {
        self.1 == other.1
    }
}

#[derive(Debug, PartialEq)]
enum ShooterType {
    Numbered(usize),
    S,
}

#[derive(Debug)]
struct Shooter {
    typ: ShooterType,
    pos: Point,
}

impl Shooter {
    fn find_targets(&self, zombies: &[Zombie]) -> Vec<usize> {
        let it = zombies
            .into_iter()
            .enumerate()
            .filter(|(_, z)| z.hp > 0 && self.pos.0 < z.pos.0);
        match self.typ {
            ShooterType::Numbered(_) => it
                .filter(|(_, z)| self.pos.is_horizontal(&z.pos))
                .min_by(|(_, a), (_, b)| a.pos.0.cmp(&b.pos.0))
                .map(|(i, _)| i)
                .into_iter()
                .collect(),
            ShooterType::S => {
                let mut ne = false;
                let mut e = false;
                let mut se = false;
                let potential_targets = it
                    // S-shooter can't fire zombies "behind" it. the hack is to sort zombies
                    // positions by X so that closest zombies (by each of three "axes") are placed
                    // closer to the vector's head
                    .sorted_by(|(_, a), (_, b)| a.pos.0.cmp(&b.pos.0));

                let mut targets = vec![];
                for (i, z) in potential_targets {
                    if !ne && self.pos.is_diagonal_ne(&z.pos) {
                        targets.push(i);
                        ne = true
                    }

                    if !se && self.pos.is_diagonal_se(&z.pos) {
                        targets.push(i);
                        se = true;
                    }

                    if !e && self.pos.is_horizontal(&z.pos) {
                        targets.push(i);
                        e = true;
                    }
                    if ne && e && se {
                        break;
                    }
                }

                targets
            }
        }
    }
}

#[derive(Debug)]
struct Zombie {
    hp: usize,
    max_hp: usize,
    pos: Point,
}

impl Zombie {
    fn as_char(&self) -> char {
        if self.hp == 0 {
            return ' ';
        }
        let percent = (10.0 * (self.hp as f32) / (self.max_hp as f32)) as u32;
        char::from_digit(percent, 10).unwrap_or('Z')
    }
}

#[derive(Debug)]
struct Spawn {
    at_tick: usize,
    row: usize,
    hp: usize,
}

impl Spawn {
    fn make_zombie(self, row_length: usize) -> Zombie {
        Zombie {
            hp: self.hp,
            max_hp: self.hp,
            pos: Point(row_length - 1, self.row),
        }
    }
}

#[derive(Debug)]
enum StepResult {
    Continue,
    NoZombiesLeft,
    Lost(usize),
}

// #[derive(Debug)]
struct Lawn {
    shooters: Vec<Shooter>,
    zombies: Vec<Zombie>,
    spawns: Vec<Spawn>,
    tick: usize,
    width: usize,
    height: usize,
    selected_shooter: Option<usize>,
    selected_targets: Option<Vec<usize>>,
}

impl Debug for Lawn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "t{:03} | {:03} shooters | {:03} zombies | {:03} spawns | left-most: {:?}",
            self.tick,
            self.shooters.len(),
            self.zombies.iter().filter(|z| z.hp > 0).count(),
            self.spawns.len(),
            self.zombies
                .iter()
                .filter(|z| z.hp > 0)
                .min_by(|a, b| a.pos.0.cmp(&b.pos.0)),
        ))
    }
}

impl Lawn {
    fn step(&mut self) -> StepResult {
        for zombie in &mut self.zombies {
            if zombie.hp == 0 {
                continue;
            }
            match zombie.pos {
                Point(0, _) => return StepResult::Lost(self.tick),
                _ => {
                    zombie.pos.0 -= 1;
                    self.shooters.retain(|x| x.pos != zombie.pos);
                }
            }
        }

        while let Some(&Spawn { at_tick, .. }) = self.spawns.last() {
            if self.tick == at_tick {
                let spawn = self.spawns.pop().unwrap();
                self.zombies.push(spawn.make_zombie(self.width))
            } else {
                break;
            }
        }

        for (i, shooter) in self.shooters.iter().enumerate() {
            match shooter.typ {
                ShooterType::Numbered(n) => {
                    self.selected_shooter = Some(i);
                    let mut rest = n;
                    while rest > 0 {
                        let targets = shooter.find_targets(&self.zombies);
                        self.selected_targets = Some(targets.clone());
                        self.pretty_print();
                        // sleep!(10);
                        if targets.len() == 0 {
                            break;
                        }
                        assert_eq!(targets.len(), 1);
                        let idx = targets[0];
                        let target = &mut self.zombies[idx];
                        let shots = target.hp.min(rest);
                        target.hp -= shots;
                        rest -= shots;
                        self.selected_targets = None;
                    }
                    self.selected_shooter = None;
                }
                _ => {}
            }
        }

        for (i, shooter) in self.shooters.iter().enumerate() {
            match shooter.typ {
                ShooterType::S => {
                    self.selected_shooter = Some(i);
                    let targets = shooter.find_targets(&self.zombies);
                    self.selected_targets = Some(targets.clone());
                    self.pretty_print();
                    // sleep!(10);
                    // if targets.len() == 3 {
                    //     sleep!(50);
                    // }
                    for idx in targets {
                        let zombie = &mut self.zombies[idx];
                        zombie.hp -= 1;
                    }
                    self.selected_targets = None;
                    self.selected_shooter = None;
                }
                _ => {}
            }
        }
        self.pretty_print();
        // sleep!(250);

        if self.spawns.len() == 0 && self.zombies.iter().all(|z| z.hp == 0) {
            return StepResult::NoZombiesLeft;
        }

        self.tick += 1;
        StepResult::Continue
    }

    #[allow(unreachable_code)]
    fn pretty_print(&self) {
        return;
        assert!(std::process::Command::new("cls")
            .status()
            .or_else(|_| std::process::Command::new("clear").status())
            .unwrap()
            .success());
        let mut v = vec![vec!["   ".to_string(); self.width]; self.height];

        if let Some(idx) = self.selected_shooter {
            if self.shooters[idx].typ == ShooterType::S {
                let Point(pos_x, pos_y) = self.shooters[idx].pos;
                for x in pos_x..self.width {
                    v[pos_y][x] = " * ".to_string();
                }

                let mut x = pos_x;
                let mut y = pos_y;
                loop {
                    if y == 0 {
                        break;
                    }
                    x += 1;
                    y -= 1;
                    if x == self.width || y == 0 {
                        break;
                    }
                    v[y][x] = " * ".to_string()
                }

                let mut x = pos_x;
                let mut y = pos_y;
                loop {
                    x += 1;
                    y += 1;
                    if x == self.width || y == self.height {
                        break;
                    }
                    v[y][x] = " * ".to_string();
                }
            }
        }

        for (i, s) in self.shooters.iter().enumerate() {
            let Point(x, y) = s.pos;
            match s.typ {
                ShooterType::Numbered(n) => v[y][x] = format!(" {n}-"),
                ShooterType::S => v[y][x] = " S<".to_string(),
            };
            if let Some(idx) = self.selected_shooter {
                if idx == i {
                    v[y][x] = format!(
                        "[{}]",
                        match s.typ {
                            ShooterType::Numbered(n) => n.to_string(),
                            ShooterType::S => "S".to_string(),
                        }
                    );
                }
            }
        }

        for (i, z) in self.zombies.iter().enumerate() {
            if z.hp == 0 {
                continue;
            }
            let Point(x, y) = z.pos;
            v[y][x] = format!(" {} ", z.as_char());
            if let Some(targets) = &self.selected_targets {
                if targets.contains(&i) {
                    v[y][x] = format!("[{}]", z.as_char());
                }
            }
        }

        let mut result = format!("==== {:03} ====\n", self.tick);
        result.push_str(&v.into_iter().map(|row| row.iter().join("")).join("\n"));
        let dead_zombies = self.zombies.iter().filter(|z| z.hp == 0).count();
        result.push_str(&format!("\ndead zombies: {}", dead_zombies));
        println!("{}", result)
    }
}

mod pnz {
    use std::cmp::Ordering;

    use itertools::Itertools;

    use crate::{Lawn, Point, Shooter, ShooterType, Spawn, StepResult};

    pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
        let row_length = lawn[0].len();
        let shooters = lawn
            .into_iter()
            .enumerate()
            .map(|(y, row)| row.chars().enumerate().map(move |(x, ch)| (x, y, ch)))
            .flatten()
            .filter_map(|(x, y, ch)| {
                let typ = match ch {
                    'S' => Some(ShooterType::S),
                    ' ' => None,
                    v => Some(ShooterType::Numbered(v.to_digit(10).unwrap() as usize)),
                };
                typ.map(|typ| Shooter {
                    typ,
                    pos: Point(x, y),
                })
            })
            .sorted_by(|a, b| match b.pos.0.cmp(&a.pos.0) {
                Ordering::Equal => a.pos.1.cmp(&b.pos.1),
                v => v,
            })
            .collect_vec();

        let spawns = zombies
            .into_iter()
            .map(|info| Spawn {
                at_tick: info[0],
                row: info[1],
                hp: info[2],
            })
            .sorted_by(|a, b| b.at_tick.cmp(&a.at_tick)) // reversed order
            .collect_vec();

        let mut game = Lawn {
            shooters,
            spawns,
            width: row_length,
            height: lawn.len(),
            selected_shooter: None,
            zombies: vec![],
            tick: 0,
            selected_targets: None,
        };

        loop {
            match game.step() {
                StepResult::Continue => {}
                StepResult::NoZombiesLeft => {
                    println!("no zombies left");
                    eprintln!("{lawn:?}");
                    eprintln!("{zombies:?}");
                    return 0;
                }
                StepResult::Lost(step) => {
                    println!("no shooters left");
                    eprintln!("{lawn:?}");
                    eprintln!("{zombies:?}");
                    return step;
                }
            }
        }
    }
}

fn main() {
    let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![(
        vec![
            "112SSS            ",
            "7                 ",
            "4S1S              ",
            "1S1S              ",
            "S14S              ",
            "7S                ",
            "2                 ",
            "3S111             ",
            "S 2   14          ",
            "5S2               ",
            "5 S               ",
            "1S3111  1         ",
            "2 1S1S1 1         ",
            "2S21S S           ",
            "133               ",
            "S121 S S          ",
            "S1SS2             ",
        ],
        vec![
            vec![2, 0, 93],
            vec![2, 2, 93],
            vec![2, 3, 53],
            vec![2, 5, 106],
            vec![2, 6, 26],
            vec![2, 7, 93],
            vec![2, 8, 106],
            vec![2, 9, 106],
            vec![2, 10, 79],
            vec![2, 11, 119],
            vec![2, 12, 106],
            vec![2, 13, 106],
            vec![2, 14, 93],
            vec![2, 16, 79],
            vec![4, 4, 98],
            vec![4, 15, 98],
            vec![5, 1, 103],
            vec![8, 0, 50],
            vec![8, 5, 57],
            vec![8, 6, 14],
            vec![8, 7, 50],
            vec![8, 10, 43],
            vec![8, 12, 57],
            vec![8, 13, 57],
            vec![8, 16, 43],
            vec![9, 1, 48],
            vec![9, 2, 55],
            vec![9, 4, 51],
            vec![9, 8, 63],
            vec![9, 15, 51],
            vec![12, 3, 37],
            vec![12, 11, 85],
            vec![12, 14, 65],
            vec![13, 9, 81],
            vec![17, 0, 49],
            vec![17, 5, 56],
            vec![17, 6, 14],
            vec![17, 10, 42],
            vec![17, 13, 56],
            vec![17, 16, 42],
            vec![19, 1, 48],
            vec![19, 2, 51],
            vec![19, 12, 62],
            vec![19, 15, 50],
            vec![20, 3, 28],
            vec![20, 4, 55],
            vec![20, 7, 59],
            vec![20, 8, 64],
            vec![20, 14, 48],
            vec![23, 9, 62],
            vec![24, 11, 82],
            vec![26, 6, 14],
            vec![26, 10, 42],
            vec![26, 13, 56],
            vec![27, 0, 54],
            vec![27, 5, 62],
            vec![27, 7, 47],
            vec![27, 16, 47],
            vec![32, 1, 54],
            vec![32, 2, 54],
            vec![32, 3, 28],
            vec![35, 12, 70],
            vec![35, 14, 54],
            vec![35, 15, 59],
            vec![36, 4, 61],
            vec![36, 8, 70],
            vec![36, 9, 58],
            vec![36, 11, 61],
            vec![39, 6, 14],
            vec![39, 10, 42],
            vec![39, 13, 56],
            vec![40, 5, 58],
            vec![40, 7, 48],
            vec![40, 16, 43],
            vec![44, 0, 56],
            vec![44, 1, 51],
            vec![44, 3, 28],
            vec![44, 12, 54],
            vec![44, 15, 47],
            vec![45, 2, 56],
            vec![45, 14, 51],
            vec![46, 9, 56],
            vec![47, 4, 57],
            vec![47, 11, 69],
            vec![48, 8, 71],
            vec![52, 6, 14],
            vec![52, 10, 42],
            vec![52, 13, 56],
            vec![54, 5, 56],
            vec![54, 7, 49],
            vec![54, 16, 42],
            vec![55, 0, 51],
            vec![55, 3, 28],
            vec![55, 15, 48],
            vec![56, 1, 54],
            vec![56, 12, 61],
            vec![56, 14, 49],
            vec![57, 2, 56],
            vec![58, 4, 51],
        ],
        27,
    )];

    example_tests.into_iter().for_each(|(grid, zqueue, sol)| {
        let res = pnz::plants_and_zombies(&grid, &zqueue);
        if res == sol {
            println!("solved")
        } else {
            println!("not solved")
        }
    });
}

#[cfg(test)]
mod example_tests {
    use crate::pnz;

    use super::*;

    #[test]
    fn example_tests() {
        let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![
            (
                vec!["2       ", "  S     ", "21  S   ", "13      ", "2 3     "],
                vec![
                    vec![0, 4, 28],
                    vec![1, 1, 6],
                    vec![2, 0, 10],
                    vec![2, 4, 15],
                    vec![3, 2, 16],
                    vec![3, 3, 13],
                ],
                10,
            ),
            (
                vec!["11      ", " 2S     ", "11S     ", "3       ", "13      "],
                vec![
                    vec![0, 3, 16],
                    vec![2, 2, 15],
                    vec![2, 1, 16],
                    vec![4, 4, 30],
                    vec![4, 2, 12],
                    vec![5, 0, 14],
                    vec![7, 3, 16],
                    vec![7, 0, 13],
                ],
                12,
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         ",
                ],
                vec![
                    vec![0, 0, 18],
                    vec![2, 3, 12],
                    vec![2, 5, 25],
                    vec![4, 2, 21],
                    vec![6, 1, 35],
                    vec![6, 4, 9],
                    vec![8, 0, 22],
                    vec![8, 1, 8],
                    vec![8, 2, 17],
                    vec![10, 3, 18],
                    vec![11, 0, 15],
                    vec![12, 4, 21],
                ],
                20,
            ),
            (
                vec!["12      ", "2S      ", "1S      ", "2S      ", "3       "],
                vec![
                    vec![0, 0, 15],
                    vec![1, 1, 18],
                    vec![2, 2, 14],
                    vec![3, 3, 15],
                    vec![4, 4, 13],
                    vec![5, 0, 12],
                    vec![6, 1, 19],
                    vec![7, 2, 11],
                    vec![8, 3, 17],
                    vec![9, 4, 18],
                    vec![10, 0, 15],
                    vec![11, 4, 14],
                ],
                19,
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         ",
                ],
                vec![
                    vec![0, 2, 16],
                    vec![1, 3, 19],
                    vec![2, 0, 18],
                    vec![4, 2, 21],
                    vec![6, 3, 20],
                    vec![7, 5, 17],
                    vec![8, 1, 21],
                    vec![8, 2, 11],
                    vec![9, 0, 10],
                    vec![11, 4, 23],
                    vec![12, 1, 15],
                    vec![13, 3, 22],
                ],
                0,
            ),
            (
                vec![
                    " 41                   ",
                    "13SS                  ",
                    "22 S                  ",
                    "2S1S                  ",
                    " 4                    ",
                    "2S1S                  ",
                    "25  SSS               ",
                    "511                   ",
                    "11S1  S               ",
                    "1 1  SSS  1           ",
                    "31  1S S              ",
                    "11  1                 ",
                    "51SSS                 ",
                    "2                     ",
                    "111SS                 ",
                    "3SS    SS             ",
                    "2S4S 1                ",
                    "113SS S               ",
                    " 4 SS                 ",
                ],
                vec![
                    vec![0, 0, 81],
                    vec![0, 1, 97],
                    vec![0, 3, 81],
                    vec![0, 4, 65],
                    vec![0, 5, 81],
                    vec![0, 6, 162],
                    vec![0, 8, 81],
                    vec![0, 10, 113],
                    vec![0, 11, 48],
                    vec![0, 12, 146],
                    vec![0, 13, 32],
                    vec![0, 14, 81],
                    vec![0, 18, 97],
                    vec![1, 2, 85],
                    vec![1, 9, 102],
                    vec![1, 15, 119],
                    vec![1, 16, 153],
                    vec![1, 17, 136],
                    vec![3, 7, 124],
                    vec![7, 0, 43],
                    vec![7, 1, 52],
                    vec![7, 4, 34],
                    vec![7, 6, 87],
                    vec![7, 8, 43],
                    vec![7, 10, 61],
                    vec![7, 11, 26],
                    vec![7, 13, 17],
                    vec![7, 14, 43],
                    vec![7, 18, 52],
                    vec![8, 7, 58],
                    vec![8, 9, 53],
                    vec![8, 12, 85],
                    vec![8, 15, 62],
                    vec![8, 16, 79],
                    vec![8, 17, 71],
                    vec![9, 2, 48],
                    vec![9, 3, 51],
                    vec![9, 5, 51],
                    vec![17, 1, 49],
                    vec![17, 4, 33],
                    vec![17, 8, 41],
                    vec![17, 10, 57],
                    vec![17, 11, 25],
                    vec![17, 13, 17],
                    vec![17, 18, 49],
                    vec![19, 6, 89],
                    vec![19, 12, 75],
                    vec![19, 14, 45],
                    vec![19, 15, 57],
                    vec![19, 16, 74],
                    vec![19, 17, 65],
                    vec![20, 0, 48],
                    vec![20, 2, 42],
                    vec![20, 3, 42],
                    vec![20, 5, 42],
                    vec![20, 9, 54],
                    vec![21, 7, 67],
                    vec![32, 4, 35],
                    vec![32, 6, 82],
                    vec![32, 8, 44],
                    vec![32, 11, 26],
                    vec![32, 13, 17],
                    vec![32, 14, 41],
                    vec![32, 18, 53],
                    vec![33, 3, 41],
                    vec![33, 5, 41],
                    vec![33, 9, 49],
                    vec![33, 16, 79],
                    vec![34, 1, 62],
                    vec![34, 10, 72],
                    vec![34, 12, 86],
                    vec![34, 15, 67],
                    vec![34, 17, 76],
                    vec![39, 0, 54],
                    vec![39, 7, 69],
                    vec![42, 2, 55],
                    vec![46, 4, 33],
                    vec![46, 8, 41],
                    vec![46, 11, 25],
                    vec![46, 13, 17],
                    vec![46, 18, 49],
                    vec![49, 1, 51],
                    vec![49, 6, 96],
                    vec![49, 9, 53],
                    vec![49, 10, 60],
                    vec![49, 12, 76],
                    vec![49, 14, 48],
                    vec![49, 17, 68],
                ],
                30,
            ),
        ];

        example_tests.into_iter().for_each(|(grid, zqueue, sol)| {
            let res = pnz::plants_and_zombies(&grid, &zqueue);
            if res == sol {
                println!("solved")
            } else {
                println!("not solved")
            }
            assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol)
        });
    }
}
