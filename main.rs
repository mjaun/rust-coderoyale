use std::io;

include!("point.rs");
include!("owner.rs");
include!("unit.rs");
include!("site.rs");
include!("task.rs");
include!("context.rs");
include!("command.rs");

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut context = Context::new();
    let mut task: Box<dyn Task> = Box::new(WaitTask {});
    let mut first_iteration = true;
    let mut loop_count = 0;
    let mut input_lines: Vec<String> = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_lines.push(String::from(input_line.trim()));
    let num_sites = parse_input!(input_line, i32);

    for _ in 0..num_sites as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_lines.push(String::from(input_line.trim()));
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let site_id = parse_input!(inputs[0], i32);
        let x = parse_input!(inputs[1], i32);
        let y = parse_input!(inputs[2], i32);
        let radius = parse_input!(inputs[3], i32);

        context.sites.push(Site {
            id: site_id,
            position: Vector2 {
                x: x as f64,
                y: y as f64,
            },
            radius,
            max_mining_rate: -1,
            gold: -1,
            structure: Structure::NoStructure,
        });
    }

    // game loop
    loop {
        context.units.clear();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_lines.push(String::from(input_line.trim()));
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gold = parse_input!(inputs[0], i32);
        let touched_site = parse_input!(inputs[1], i32); // -1 if none

        for _ in 0..num_sites as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            input_lines.push(String::from(input_line.trim()));
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], i32);
            let gold = parse_input!(inputs[1], i32);
            let max_mining_rate = parse_input!(inputs[2], i32);
            let structure_type = parse_input!(inputs[3], i32);
            let owner = parse_input!(inputs[4], i32);
            let param_1 = parse_input!(inputs[5], i32);
            let param_2 = parse_input!(inputs[6], i32);

            let owner: Option<Owner> = match owner {
                0 => Some(Owner::Friendly),
                1 => Some(Owner::Enemy),
                _ => None,
            };

            let site = context.site_by_id_mut(site_id);
            site.gold = gold;
            site.max_mining_rate = max_mining_rate;

            site.structure = match structure_type {
                -1 => Structure::NoStructure,
                0 => Structure::Mine(MineStructure {
                    owner: owner.unwrap(),
                    mining_rate: param_1,
                }),
                1 => Structure::Tower(TowerStructure {
                    owner: owner.unwrap(),
                    health: param_1,
                    attack_radius: param_2,
                }),
                2 => Structure::Barracks(BarracksStructure {
                    owner: owner.unwrap(),
                    barracks_type: match param_2 {
                        0 => BarracksType::Knight,
                        1 => BarracksType::Archer,
                        2 => BarracksType::Giant,
                        _ => panic!(),
                    },
                    turns_to_train: param_1,
                }),
                _ => panic!(),
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_lines.push(String::from(input_line.trim()));
        let num_units = parse_input!(input_line, i32);

        for _ in 0..num_units as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            input_lines.push(String::from(input_line.trim()));
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let owner = parse_input!(inputs[2], i32);
            let unit_type = parse_input!(inputs[3], i32);
            let health = parse_input!(inputs[4], i32);

            context.units.push(Unit {
                position: Vector2 {
                    x: x as f64,
                    y: y as f64,
                },
                owner: match owner {
                    0 => Owner::Friendly,
                    1 => Owner::Enemy,
                    _ => panic!(),
                },
                unit_type: match unit_type {
                    -1 => UnitType::Queen,
                    0 => UnitType::Knight,
                    1 => UnitType::Archer,
                    2 => UnitType::Giant,
                    _ => panic!(),
                },
                health,
            });
        }

        if first_iteration {
            context.start_position = context.friendly_queen().position;
            first_iteration = false;
        }

        context.gold = gold;
        context.touched_site_id = touched_site;

        if task.is_task_done(&context) {
            task = get_next_task(&context);
        }

        println!("{}", task.get_next_command(&context));
        println!("{}", get_train_command(&context));

        for line in &input_lines {
            eprint!("{};", line);
        }
        eprintln!("");
        input_lines.clear();

        loop_count += 1;
    }
}

