struct Context {
    sites: Vec<Site>,
    units: Vec<Unit>,
    gold: i32,
    start_position: Vector2,
    touched_site_id: i32,
}

impl Context {
    fn new() -> Context {
        Context {
            sites: Vec::new(),
            units: Vec::new(),
            gold: 0,
            start_position: Vector2 {
                x: 0.0,
                y: 0.0,
            },
            touched_site_id: -1,
        }
    }

    fn site_by_id(&self, site_id: i32) -> &Site {
        self.sites.iter()
            .find(|site| site.id == site_id)
            .unwrap()
    }

    fn site_by_id_mut(&mut self, site_id: i32) -> &mut Site {
        self.sites.iter_mut()
            .find(|site| site.id == site_id)
            .unwrap()
    }

    fn friendly_queen(&self) -> &Unit {
        self.units.iter()
            .find(|unit| unit.unit_type == UnitType::Queen && unit.owner == Owner::Friendly)
            .unwrap()
    }

    fn site_count(&self, build_type: BuildType, owner: Owner) -> usize {
        self.sites.iter()
            .filter(|site| {
                match &site.structure {
                    Structure::Barracks(barracks) => {
                        match build_type {
                            BuildType::Barracks(barracks_type) => {
                                barracks.barracks_type == barracks_type && barracks.owner == owner
                            },
                            _ => false,
                        }
                    }
                    Structure::Mine(mine) => build_type == BuildType::Mine && mine.owner == owner,
                    Structure::Tower(tower) => build_type == BuildType::Tower && tower.owner == owner,
                    Structure::NoStructure => false,
                }
            })
            .count()
    }

    fn unit_count(&self, unit_type: UnitType, owner: Owner) -> usize {
        self.units.iter()
            .filter(|unit| unit.unit_type == unit_type && unit.owner == owner)
            .count()
    }

    fn closest_unit_dist(&self, unit_type: UnitType, owner: Owner) -> Option<i32> {
        let reference_point = self.friendly_queen().position;

        self.units.iter()
            .filter(|unit| unit.unit_type == unit_type && unit.owner == owner)
            .map(|unit| (unit.position - reference_point).len() as i32)
            .min()
    }

    fn closest_site(&self, predicate: impl Fn(&Site) -> bool) -> Option<&Site> {
        let reference_point = self.friendly_queen().position;

        self.sites.iter()
            .filter(|site| predicate(site))
            .min_by_key(|site| (site.position - reference_point).len() as i32)
    }
}
