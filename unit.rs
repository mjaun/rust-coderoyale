#[derive(PartialEq, Eq)]
enum UnitType {
    Queen,
    Knight,
    Archer,
    Giant,
}

struct Unit {
    position: Vector2,
    owner: Owner,
    unit_type: UnitType,
    health: i32,
}

