pub mod cave;
pub mod dungeon;
pub mod interior;
pub mod overworld;

pub trait Stage: Sync {
    fn get_id(&self) -> &'static str;
    fn get_rooms(&self) -> &'static [Room<'static>];
    fn len(&self) -> usize;
}

pub struct Room<'a> {
    pub name: &'a str,
    pub id: u8,
    pub spawn_points: &'a [u8],
}

impl<'a> Room<'a> {
    pub const fn new(name: &'a str, id: u8, spawn_points: &'a [u8]) -> Self {
        Room {
            name,
            id,
            spawn_points,
        }
    }
}
