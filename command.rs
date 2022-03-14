use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
enum BuildType {
    Barracks(BarracksType),
    Tower,
    Mine,
}

struct BuildCommand {
    site_id: i32,
    build_type: BuildType,
}

enum QueenCommand {
    Wait,
    Move(Vector2),
    Build(BuildCommand),
}

struct TrainCommand {
    site_ids: Vec<i32>,
}

impl BuildCommand {
    fn for_site(site: &Site, build_type: BuildType) -> BuildCommand {
        BuildCommand {
            site_id: site.id,
            build_type,
        }
    }
}

impl TrainCommand {
    fn for_sites(sites: Vec<&Site>) -> TrainCommand {
        TrainCommand {
            site_ids: sites.iter().map(|site| site.id).collect(),
        }
    }
}

impl fmt::Display for QueenCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueenCommand::Wait => write!(f, "WAIT"),
            QueenCommand::Move(position) => {
                write!(f, "MOVE {} {}", position.x as i32, position.y as i32)
            },
            QueenCommand::Build(build_cmd) => {
                let type_str = match &build_cmd.build_type {
                    BuildType::Barracks(barracks_type) => {
                        match barracks_type {
                            BarracksType::Knight => "BARRACKS-KNIGHT",
                            BarracksType::Archer => "BARRACKS-ARCHER",
                            BarracksType::Giant => "BARRACKS-GIANT",
                        }
                    },
                    BuildType::Mine => "MINE",
                    BuildType::Tower => "TOWER",
                };
                write!(f, "BUILD {} {}", build_cmd.site_id, type_str)
            }
        }
    }
}

impl fmt::Display for TrainCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ids_str = String::from("TRAIN");
        for site_id in &self.site_ids {
            ids_str.push_str(" ");
            ids_str.push_str(&site_id.to_string());
        }
        write!(f, "{}", ids_str)
    }
}


fn get_next_task(context: &Context) -> Box<dyn Task> {
    let build_site = get_closest_site_to_build(context);

    if build_site.is_none() {
        return Box::new(WaitTask::new());
    }

    let build_site = build_site.unwrap();

    if context.touched_site_id != build_site.id {
        return Box::new(MoveToBuildSiteTask::new(build_site.id));
    }

    let friendly_tower_count = context.site_count(BuildType::Tower, Owner::Friendly);
    let friendly_mine_count = context.site_count(BuildType::Mine, Owner::Friendly);
    let friendly_knight_barracks_count = context.site_count(BuildType::Barracks(BarracksType::Knight), Owner::Friendly);
    let friendly_giant_barracks_count = context.site_count(BuildType::Barracks(BarracksType::Giant), Owner::Friendly);
    let enemy_tower_count = context.site_count(BuildType::Tower, Owner::Enemy);
    let enemy_knight_count = context.unit_count(UnitType::Knight, Owner::Enemy);
    let enemy_knight_dist = context.closest_unit_dist(UnitType::Knight, Owner::Enemy);

    // build tower if there are enemy knights and we have no towers
    if enemy_knight_count > 0 && friendly_tower_count == 0 {
        return Box::new(BuildTowerTask::new(build_site));
    }

    // build a minimum number of mines
    if friendly_mine_count < 3 && enemy_knight_dist.unwrap_or(i32::MAX) > 300 {
        return Box::new(BuildMineTask::new(build_site));
    }

    // build knights barracks
    if friendly_knight_barracks_count == 0 {
        return Box::new(BuildBarracksTask::new(build_site, BarracksType::Knight));
    }

    // build a minimum amount of towers
    if friendly_tower_count < 2 {
        return Box::new(BuildTowerTask::new(build_site));
    }

    // build giant barracks if enemy builds towers
    if enemy_tower_count > 0 && friendly_giant_barracks_count == 0 {
        return Box::new(BuildBarracksTask::new(build_site, BarracksType::Giant));
    }

    // build an additional amount of mines if no enemy knight is close
    if friendly_mine_count < 5 && enemy_knight_dist.unwrap_or(i32::MAX) > 300 {
        return Box::new(BuildMineTask::new(build_site));
    }

    return Box::new(BuildTowerTask::new(build_site));
}

fn get_next_unit_to_train(context: &Context) -> Option<BarracksType> {
    let enemy_tower_count = context.site_count(BuildType::Tower, Owner::Enemy);
    let friendly_knight_barracks_count = context.site_count(BuildType::Barracks(BarracksType::Knight), Owner::Friendly);
    let friendly_giant_barracks_count = context.site_count(BuildType::Barracks(BarracksType::Giant), Owner::Friendly);
    let friendly_giant_count = context.unit_count(UnitType::Giant, Owner::Friendly);

    if enemy_tower_count > 0 && friendly_giant_barracks_count > 0 && friendly_giant_count == 0 {
        return Some(BarracksType::Giant);
    }

    if friendly_knight_barracks_count > 0 {
        return Some(BarracksType::Knight);
    }

    return None;
}

fn get_closest_site_to_build(context: &Context) -> Option<&Site> {
    context.closest_site(|site| {
        if (site.position - context.start_position).len() > 1000.0 {
            return false;
        }

        let enemy_tower_in_range = context.sites.iter().any(|site2| {
            site2.is_tower() &&
                site2.tower().owner == Owner::Enemy &&
                (site2.position - site.position).len() < site2.tower().attack_radius as f64
        });

        if enemy_tower_in_range {
            return false;
        }

        match &site.structure {
            Structure::Barracks(barracks) => barracks.owner != Owner::Friendly,
            Structure::Mine(mine) => mine.owner != Owner::Friendly,
            Structure::Tower(_) => false,
            Structure::NoStructure => true,
        }
    })
}

fn get_train_command(context: &Context) -> TrainCommand {
    let mut gold_available = context.gold;
    let mut train_sites: Vec<&Site> = Vec::new();

    while let Some(barracks_type) = get_next_unit_to_train(context) {
        if gold_available < barracks_type.cost_to_train() {
            break;
        }

        let train_site = context.sites.iter().find(|site| {
            site.is_barracks()
                && site.owner().unwrap() == Owner::Friendly
                && site.barracks().barracks_type == barracks_type
                && site.barracks().turns_to_train == 0
                && !train_sites.iter().any(|site2| site.id == site2.id)
        });

        if let Some(train_site) = train_site {
            train_sites.push(train_site);
            gold_available -= barracks_type.cost_to_train();
        } else {
            break;
        }
    }

    TrainCommand::for_sites(train_sites)
}

