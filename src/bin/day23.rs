use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Energy = u64;

const PART1: Situation<2> = Situation {
    hallway: [None; 11],
    rooms: [
        Room {
            colour: AmphipodType::Amber,
            spots: [Some(AmphipodType::Desert), Some(AmphipodType::Copper)],
        },
        Room {
            colour: AmphipodType::Bronze,
            spots: [Some(AmphipodType::Desert), Some(AmphipodType::Copper)],
        },
        Room {
            colour: AmphipodType::Copper,
            spots: [Some(AmphipodType::Amber), Some(AmphipodType::Bronze)],
        },
        Room {
            colour: AmphipodType::Desert,
            spots: [Some(AmphipodType::Amber), Some(AmphipodType::Bronze)],
        },
    ],
};

const PART2: Situation<4> = Situation {
    hallway: [None; 11],
    rooms: [
        Room {
            colour: AmphipodType::Amber,
            spots: [Some(AmphipodType::Desert), Some(AmphipodType::Desert), Some(AmphipodType::Desert), Some(AmphipodType::Copper)],
        },
        Room {
            colour: AmphipodType::Bronze,
            spots: [Some(AmphipodType::Desert), Some(AmphipodType::Copper), Some(AmphipodType::Bronze), Some(AmphipodType::Copper)],
        },
        Room {
            colour: AmphipodType::Copper,
            spots: [Some(AmphipodType::Amber), Some(AmphipodType::Bronze), Some(AmphipodType::Amber), Some(AmphipodType::Bronze)],
        },
        Room {
            colour: AmphipodType::Desert,
            spots: [Some(AmphipodType::Amber), Some(AmphipodType::Amber), Some(AmphipodType::Copper), Some(AmphipodType::Bronze)],
        },
    ],
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodType {
    fn energy(self) -> Energy {
        match self {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Room<const ROOM_SIZE: usize> {
    colour: AmphipodType,
    spots: [Option<AmphipodType>; ROOM_SIZE],
}

impl<const ROOM_SIZE: usize> Room<ROOM_SIZE> {
    fn is_filled(&self) -> bool {
        self.spots.into_iter().all(|c| c == Some(self.colour))
    }

    fn accept(&self, amphipod: AmphipodType) -> Option<(Energy, Self)> {
        if self.colour == amphipod && self.spots.iter().flatten().all(|&c| c == amphipod) {
            let p = self
                .spots
                .iter()
                .rposition(Option::is_none)
                .expect("No more free room.");
            let mut new_room = *self;
            new_room.spots[p] = Some(amphipod);
            Some(((p as Energy + 1) * amphipod.energy(), new_room))
        } else {
            None
        }
    }

    fn take(&self) -> Option<(Energy, AmphipodType, Self)> {
        // Don't take out an amphipod if it's already in the right room (and not
        // blocking a different one).
        if self.spots.iter().flatten().all(|&a| a == self.colour) {
            None
        } else {
            self.spots.iter().position(Option::is_some).map(|p| {
                let mut new_room = *self;
                let amphipod = new_room.spots[p].take().unwrap();
                ((p as Energy + 1) * amphipod.energy(), amphipod, new_room)
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Situation<const ROOM_SIZE: usize> {
    hallway: [Option<AmphipodType>; 11],
    rooms: [Room<ROOM_SIZE>; 4],
}

impl<const ROOM_SIZE: usize> Situation<ROOM_SIZE> {
    const POSSIBLE_SPOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
    const ROOMS_IDX: [usize; 4] = [2, 4, 6, 8];

    fn is_done(&self) -> bool {
        self.rooms.iter().all(Room::is_filled)
    }

    fn accessible_rooms(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        let left = (0..idx)
            .rev()
            .take_while(|tile| self.hallway[*tile].is_none())
            .filter(|tile| Self::ROOMS_IDX.contains(tile));
        let right = (idx + 1..11)
            .take_while(|tile| self.hallway[*tile].is_none())
            .filter(|tile| Self::ROOMS_IDX.contains(tile));

        left.chain(right)
    }

    fn accessible_tiles(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        let left = (0..idx)
            .rev()
            .take_while(|tile| self.hallway[*tile].is_none())
            .filter(|tile| Self::POSSIBLE_SPOTS.contains(tile));
        let right = (idx + 1..11)
            .take_while(|tile| self.hallway[*tile].is_none())
            .filter(|tile| Self::POSSIBLE_SPOTS.contains(tile));

        left.chain(right)
    }

    fn next_moves(&self) -> Vec<(Energy, Self)> {
        // If we can move an amphipod into the correct room, this is the optimal
        // move.
        let mut result = Vec::new();
        for idx in 0..self.hallway.len() {
            if let Some(amphipod) = self.hallway[idx] {
                for room_idx in self.accessible_rooms(idx) {
                    if let Some((energy, room)) = self.rooms[room_idx / 2 - 1].accept(amphipod) {
                        let mut new_sit = *self;
                        new_sit.hallway[idx].take();
                        new_sit.rooms[room_idx / 2 - 1] = room;
                        let cost = if idx < room_idx {
                            room_idx - idx
                        } else {
                            idx - room_idx
                        } as Energy
                            * amphipod.energy();
                        result.push((energy + cost, new_sit));
                    }
                }
            }
        }

        if !result.is_empty() {
            return result;
        }

        // Otherwise, for each room, try to take one amphipod out and spread it
        // over the possible tiles.
        for (room_idx, room) in Self::ROOMS_IDX.into_iter().zip(self.rooms) {
            if let Some((energy, amphipod, room)) = room.take() {
                for idx in self.accessible_tiles(room_idx) {
                    let mut new_sit = *self;
                    new_sit.hallway[idx] = Some(amphipod);
                    new_sit.rooms[room_idx / 2 - 1] = room;
                    let cost = if idx < room_idx {
                        room_idx - idx
                    } else {
                        idx - room_idx
                    } as Energy
                        * amphipod.energy();
                    result.push((energy + cost, new_sit));
                }
            }
        }

        result
    }
}

fn solve<const ROOM_SIZE: usize>(situation: Situation<ROOM_SIZE>) -> Energy {
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), situation));

    while let Some((Reverse(cost), current)) = to_visit.pop() {
        if current.is_done() {
            return cost;
        }

        if visited.insert(current) {
            for (c, s) in current.next_moves() {
                to_visit.push((Reverse(c + cost), s));
            }
        }
    }

    unreachable!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // What is the least energy required to organize the amphipods?
    let part1 = solve(PART1);
    println!("Part 1: {}", part1);

    // Using the initial configuration from the full diagram, what is the least
    // energy required to organize the amphipods?
    let part2 = solve(PART2);
    println!("Part 1: {}", part2);

    Ok(())
}
