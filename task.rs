trait Task {
    fn is_task_done(&self, context: &Context) -> bool;
    fn get_next_command(&self, context: &Context) -> QueenCommand;
}

struct BuildMineTask {
    site_id: i32,
}

struct BuildBarracksTask {
    site_id: i32,
    barracks_type: BarracksType,
}

struct BuildTowerTask {
    site_id: i32,
}

struct MoveToBuildSiteTask {
    site_id: i32,
}

struct WaitTask {

}

impl BuildMineTask {
    fn new(site: &Site) -> BuildMineTask {
        BuildMineTask {
            site_id: site.id,
        }
    }
}

impl Task for BuildMineTask {
    fn is_task_done(&self, context: &Context) -> bool {
        let site = context.site_by_id(self.site_id);
        let enemy_knight_dist = context.closest_unit_dist(UnitType::Knight, Owner::Enemy);

        if enemy_knight_dist.unwrap_or(i32::MAX) < 300 {
            return true; // abort
        }

        match &site.structure {
            Structure::Mine(mine) => {
                mine.owner == Owner::Friendly && mine.mining_rate == site.max_mining_rate
            },
            Structure::Tower(_) => true,  // abort
            _ => false,
        }
    }

    fn get_next_command(&self, _context: &Context) -> QueenCommand {
        QueenCommand::Build(BuildCommand {
            build_type: BuildType::Mine,
            site_id: self.site_id,
        })
    }
}

impl BuildBarracksTask {
    fn new(site: &Site, barracks_type: BarracksType) -> BuildBarracksTask {
        BuildBarracksTask {
            site_id: site.id,
            barracks_type,
        }
    }
}

impl Task for BuildBarracksTask {
    fn is_task_done(&self, context: &Context) -> bool {
        let site = context.site_by_id(self.site_id);

        match &site.structure {
            Structure::Barracks(barracks) => {
                barracks.owner == Owner::Friendly && barracks.barracks_type == self.barracks_type
            },
            Structure::Tower(_) => true,  // abort
            _ => false,
        }
    }

    fn get_next_command(&self, _context: &Context) -> QueenCommand {
        QueenCommand::Build(BuildCommand {
            build_type: BuildType::Barracks(self.barracks_type),
            site_id: self.site_id,
        })
    }
}

impl BuildTowerTask {
    fn new(site: &Site) -> BuildTowerTask {
        BuildTowerTask {
            site_id: site.id,
        }
    }
}

impl Task for BuildTowerTask {
    fn is_task_done(&self, context: &Context) -> bool {
        let site = context.site_by_id(self.site_id);

        match &site.structure {
            Structure::Tower(tower) => {
                tower.attack_radius > 350 || tower.owner != Owner::Friendly
            },
            _ => false,
        }
    }

    fn get_next_command(&self, _context: &Context) -> QueenCommand {
        QueenCommand::Build(BuildCommand {
            build_type: BuildType::Tower,
            site_id: self.site_id,
        })
    }
}

impl MoveToBuildSiteTask {
    fn new(site_id: i32) -> MoveToBuildSiteTask {
        MoveToBuildSiteTask {
            site_id
        }
    }
}

impl Task for MoveToBuildSiteTask {
    fn is_task_done(&self, context: &Context) -> bool {
        let site = context.site_by_id(self.site_id);
        let queen_position = context.friendly_queen().position;

        if site.is_tower() {
            return true;  // abort
        }

        const QUEEN_SPEED: f64 = 60.0;
        const QUEEN_RADIUS: f64 = 30.0;

        let dist_to_target = (site.position - queen_position).len() - (site.radius as f64) - QUEEN_RADIUS;

        eprintln!("touched site: {}", context.touched_site_id);
        eprintln!("dist to target: {}", dist_to_target as i32);

        dist_to_target < QUEEN_SPEED + 10.0
        //context.touched_site_id == self.site_id
    }

    fn get_next_command(&self, context: &Context) -> QueenCommand {
        let target_site = context.site_by_id(self.site_id);

        if context.touched_site_id >= 0 {
            let touched_site = context.site_by_id(context.touched_site_id);
            let queen_position = context.friendly_queen().position;

            let to_touched_site = touched_site.position - queen_position;
            let to_target_site = target_site.position - queen_position;

            let threshold = (20.0 as f64).to_radians();

            if Vector2::angle(to_touched_site, to_target_site) < threshold {
                let around_cw = to_touched_site.perp_cw().norm();
                let around_ccw = to_touched_site.perp_ccw().norm();

                let angle_cw = Vector2::angle(to_target_site, around_cw);
                let angle_ccw = Vector2::angle(to_target_site, around_ccw);

                return if angle_cw < angle_ccw {
                    QueenCommand::Move(queen_position + around_cw.mul(100.0))
                } else {
                    QueenCommand::Move(queen_position + around_ccw.mul(100.0))
                }
            }
        }

        QueenCommand::Move(target_site.position)
    }
}

impl WaitTask {
    fn new() -> WaitTask {
        WaitTask {}
    }
}

impl Task for WaitTask {
    fn is_task_done(&self, _context: &Context) -> bool {
        true
    }

    fn get_next_command(&self, _context: &Context) -> QueenCommand {
        QueenCommand::Wait
    }
}

