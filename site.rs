#[derive(Copy, Clone, PartialEq, Eq)]
enum BarracksType {
    Knight,
    Archer,
    Giant,
}

struct BarracksStructure {
    owner: Owner,
    barracks_type: BarracksType,
    turns_to_train: i32,
}

struct TowerStructure {
    owner: Owner,
    health: i32,
    attack_radius: i32,
}

struct MineStructure {
    owner: Owner,
    mining_rate: i32,
}

enum Structure {
    NoStructure,
    Barracks(BarracksStructure),
    Tower(TowerStructure),
    Mine(MineStructure),
}

struct Site {
    id: i32,
    position: Vector2,
    radius: i32,
    gold: i32,
    max_mining_rate: i32,
    structure: Structure,
}

impl BarracksType {
    fn cost_to_train(&self) -> i32 {
        match self {
            BarracksType::Knight => 80,
            BarracksType::Archer => 100,
            BarracksType::Giant => 140,
        }
    }
}

impl Site {
    fn is_mine(&self) -> bool {
        match &self.structure {
            Structure::Mine(_) => true,
            _ => false,
        }
    }

    fn mine(&self) -> &MineStructure {
        match &self.structure {
            Structure::Mine(mine) => mine,
            _ => panic!(),
        }
    }

    fn is_barracks(&self) -> bool {
        match &self.structure {
            Structure::Barracks(_) => true,
            _ => false,
        }
    }

    fn barracks(&self) -> &BarracksStructure {
        match &self.structure {
            Structure::Barracks(barracks) => barracks,
            _ => panic!(),
        }
    }

    fn is_tower(&self) -> bool {
        match &self.structure {
            Structure::Tower(_) => true,
            _ => false,
        }
    }

    fn tower(&self) -> &TowerStructure {
        match &self.structure {
            Structure::Tower(tower) => tower,
            _ => panic!(),
        }
    }

    fn owner(&self) -> Option<Owner> {
        match &self.structure {
            Structure::Mine(mine) => Some(mine.owner),
            Structure::Barracks(barracks) => Some(barracks.owner),
            Structure::Tower(tower) => Some(tower.owner),
            Structure::NoStructure => None,
        }
    }
}

