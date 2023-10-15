mod pnz {
    use std::cmp::Ordering;

    use itertools::Itertools;

    #[derive(Eq, PartialEq, Copy, Clone)]
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

    enum ShooterType {
        Numbered(usize),
        S,
    }

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

    struct Zombie {
        hp: usize,
        max_hp: usize,
        pos: Point,
    }

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

    enum StepResult {
        Continue,
        NoZombiesLeft,
        Lost(usize),
    }

    struct Lawn {
        shooters: Vec<Shooter>,
        zombies: Vec<Zombie>,
        spawns: Vec<Spawn>,
        tick: usize,
        width: usize,
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
                        let mut rest = n;
                        while rest > 0 {
                            let targets = shooter.find_targets(&self.zombies);
                            if targets.len() == 0 {
                                break;
                            }
                            assert_eq!(targets.len(), 1);
                            let idx = targets[0];
                            let target = &mut self.zombies[idx];
                            let shots = target.hp.min(rest);
                            target.hp -= shots;
                            rest -= shots;
                        }
                    }
                    _ => {}
                }
            }

            for (i, shooter) in self.shooters.iter().enumerate() {
                match shooter.typ {
                    ShooterType::S => {
                        let targets = shooter.find_targets(&self.zombies);
                        for idx in targets {
                            let zombie = &mut self.zombies[idx];
                            zombie.hp -= 1;
                        }
                    }
                    _ => {}
                }
            }

            if self.spawns.len() == 0 && self.zombies.iter().all(|z| z.hp == 0) {
                return StepResult::NoZombiesLeft;
            }

            self.tick += 1;
            StepResult::Continue
        }
    }

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
            zombies: vec![],
            tick: 0,
        };

        loop {
            match game.step() {
                StepResult::Continue => {}
                StepResult::NoZombiesLeft => {
                    return 0;
                }
                StepResult::Lost(step) => {
                    return step;
                }
            }
        }
    }
}
