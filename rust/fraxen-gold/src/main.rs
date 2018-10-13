
////////////////////////////////////////

struct P2 {
    x: i32,
    y: i32,
}

impl P2 {
}

////////////////////////////////////////

struct R2 {
    bl: P2,
    tr: P2,
}

impl R2 {
}

////////////////////////////////////////

pub struct Rng {
}

impl Rng {
}

////////////////////////////////////////

pub struct Die {
    n: i32,
    d: i32,
    b: i32,
}

impl Die {
}

////////////////////////////////////////

pub struct Weapon {
    dam: Die,
    tohit: i32,
}

pub struct Armor {
    ac: i32,
    dr: i32,
}

pub struct Potion {
}

pub struct Ring {
}

pub struct Wand {
    charges: i32,
}

pub struct Scroll {
}

pub struct Amulet {
}

pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Potion(Potion),
    Ring(Ring),
    Wand(Wand),
    Scroll(Scroll),
    Amulet(Amulet),
}

////////////////////////////////////////

pub struct Stat {
    curr: i32,
    max: i32,
    base: i32,
}

impl Stat {
}

////////////////////////////////////////

pub struct Entity {
    hp: Stat,
    mp: Stat,

    str: Stat,
    dex: Stat,
    con: Stat,
    int: Stat,
    wis: Stat,

    level: Stat,
    xp: Stat,

    loc: P2,

    wielded: Option<Weapon>,
    worn: Option<Armor>,
    carried: Vec<Item>,
}

impl Entity {
}

////////////////////////////////////////

pub struct Monster {
    entity: Entity,
}

////////////////////////////////////////

pub struct Player {
    entity: Entity,
}

////////////////////////////////////////

fn main() {
}


