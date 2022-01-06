#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, Default)]
pub struct BowlingGame {
    throws: Vec<u16>,
    second: bool,
}
impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second && pins + self.throws.last().unwrap() > 10) {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.throws.push(pins);
            self.second = if pins != 10 { !self.second } else { false };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frame = 0;
        let throws = &self.throws;

        for _ in 0..10 {
            if let (Some(&first), Some(&second)) = (throws.get(frame), throws.get(frame + 1)) {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(&third) = throws.get(frame + 2) {
                        total += third;
                    } else {
                        return None;
                    }
                }
                frame += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }
        Some(total)
    }
}




// #![feature(bool_to_option)]
// #![allow(clippy::must_use_candidate)]
// #![allow(clippy::missing_errors_doc)]
//
// use std::cmp::Ordering;
//
// #[derive(Debug, PartialEq)]
// pub enum Error {
//     NotEnoughPinsLeft,
//     GameComplete,
// }
//
// // Use a 21-bits-of-information automaton to compute the score on the fly
// // so there is no allocation and no synthesis is needed at the end of the game.
// #[test]
// fn golf_automaton_size() {
//     assert_eq!(std::mem::size_of::<BowlingGame>(), 3); // (with 3 unused bits)
// }
//
// // The basic principle is that, depending on the last two rolls type
// // (open, spare or strike),
// // it can be inferred how many times to count the pins in the score.
// // Pins knocked down always count at least *once* in the score,
// // but sometimes *twice* and sometimes even *thrice*
// // (after two successive strikes),
// // so the number of pins knocked down must either be multiplied by 1, 2 or 3
// // before being added to the score.
// // Here are the different roll types.
// // (Don't use explicit enum to not perform unsafe discriminant arithmetics.)
// const OPEN: u8 = 0b01;
// const SPARE: u8 = 0b10;
// const STRIKE: u8 = 0b11;
//
// // The 'factor' automaton uses 4 bits to keep track of the last 2 roll types,
// // and determinates the multiplying factor for the currently knocked pins.
// // To keep track of the game progression, another automaton called 'progression'
// // uses 7 bits to count the number of pins consumed since the game started.
// // This includes not only the pins hit by the player,
// // but also the pins not hit and cleaned up at the end of every frame
// // As a consequence, the number of pins % 10 is the number of the current frame,
// // and the game should terminate when exactly 100 pins have been counted.
// // The progression automaton uses 1 extra bit
// // to remember whether the current frame is new (0) or whether
// // a roll has already been cast (1).
// // The 'progression' automaton is also responsible for determining
// // the current roll type (open, spare or strike).
// #[derive(Default)]
// struct Progression(
//     u8, // 1 bit raised when a roll has already been cast in the frame, 7 bits for counting pins.
// );
// // At the end of the game, one or two additional rolls are allowed
// // if the last roll was a spare or a strike.
// // When this occurs, the 4-bits long 'factor' automaton is recycled
// // into a 3-bits long 'termination' automaton.
// // The 'termination' automaton contains a 2-bits counter
// // of remaining additional rolls.
// // Additional rolls are only counted once in the score,
// // so the factor emitted by the automaton is 1,
// // except in the special case where two strikes are rolled in frames 9 and 10
// // so the first additional roll should be counted twice and the factor is 2.
// // To handle this, one special 1-bit flag is prepended to the 2-bits counter,
// // and is raised in this special situation.
// // When 'termination' becomes all zero, the game is over.
// // One extra byte is used to keep track of the score.
// // This is not sufficient because the score may go beyond 255 up to 300,
// // so one extra overflow bit is required.
// // The 'bonus' byte is responsible to keep track of the score overflow (1 bit)
// // and of the 'factor' automaton (4 bits)
// // later recycled into 'termination' (3 bits).
// struct Bonus(
//     u8, // 1 bit for score overflow, 3 unused bits, 4 bits for the 'factor/termination' automatons.
// );
// impl Default for Bonus {
//     fn default() -> Self {
//         Bonus::new()
//     }
// }
// // No additional memory is required for the bowling score to be computed on the fly.
// #[derive(Default)]
// pub struct BowlingGame {
//     score: u8,
//     bonus: Bonus,
//     progression: Progression,
// }
// // Logic for the bonus byte.
// impl Bonus {
//     fn new() -> Self {
//         #[allow(clippy::unusual_byte_groupings)]
//             // Start as if 2 open rolls were just made, and no score overflow.
//             Self(0b0_000_01_01)
//         //     ^ --- ^^ ^^__ Latest roll type (open).
//         //     |  |  ||__ Earlier roll type (open).
//         //     |  |__ Unused bits.
//         //     |__ Score overflow bit.
//     }
//     // Basic manipulation of the score overflow flag.
//     fn set_score_overflow(&mut self) {
//         self.0 |= 0b1000_0000;
//     }
//     fn does_score_overflow(&self) -> bool {
//         self.0 >= 0b1000_0000
//     }
//     fn dismiss_score_overflow(&self) -> u8 {
//         self.0 & !0b1000_0000
//     }
//     // When the player casts a new roll, record the new roll type.
//     fn record(&mut self, roll_type: u8) {
//         if self.does_score_overflow() {
//             // Leave room for the new record.
//             self.0 <<= 2;
//             // Append the new record.
//             self.0 |= roll_type;
//             // Forget history past the last record.
//             self.0 &= 0b0000_1111;
//             // Reset the overflow flag.
//             self.0 |= 0b1000_0000;
//         } else {
//             // Same, but without resetting the overflow flag.
//             self.0 = ((self.0 << 2) | roll_type) & 0b0000_1111;
//         }
//     }
//     // Determine the current multiplying factor for the number of pins hit.
//     fn read_factor(&self) -> u8 {
//         // Essentially implemented as a big match or sequential if branches,
//         // but I have been recommended avoid branching and use a lookup table instead.
//         const LOOKUP: [u8; 16] = {
//             let mut table = [0; 16];
//             macro_rules! rolls_types_to_factor {
//                 ( $( $earlier:ident then $latest:ident => $bonus:literal )+ ) => {
//                     // Reconstruct the number that corresponds
//                     // to the concatenation of these two latest roll types.
//                     $( table[($earlier * 4 + $latest) as usize] = $bonus; )+
//                 }
//             }
//             rolls_types_to_factor! {
//                 OPEN   then OPEN   => 1
//                 SPARE  then OPEN   => 1
//                 OPEN   then SPARE  => 2
//                 OPEN   then STRIKE => 2
//                 SPARE  then STRIKE => 2
//                 STRIKE then OPEN   => 2
//                 STRIKE then STRIKE => 3
//             }
//             table
//         };
//         LOOKUP[self.dismiss_score_overflow() as usize]
//     }
//     // Once 100 pins have been knocked down, the 'factor' bits are recycled.
//     fn recycle_into_termination_automaton(&mut self, nb_additional_rolls: u8) {
//         // Change the meaning of the bits layout.
//         self.0 = nb_additional_rolls
//             // Raise the special flag for double-strike in frames 9 and 10.
//             + if self.read_factor() == 3 { 0b100 } else { 0 }
//             // The score overflow flag must remain unchanged.
//             + if self.does_score_overflow() { 0b1000_0000 } else { 0 };
//     }
//     // Computation of the multiplying factor is different
//     // during termination of the game.
//     fn read_additional_factor(&mut self) -> u8 {
//         macro_rules! termination_automaton {
//             ( $( $state:literal => $($new_state:literal)? return $factor:literal)+ ) => {
//                 // Only select the 3 bits of the automaton.
//                 match self.0 % 0b1000 {
//                     $( $state => {
//                         // Set up the new state without touching the overflow flag.
//                         $( self.0 = ((self.0 >> 3) << 3) + $new_state; )?
//                         // Return the adequate multiplying factor.
//                         $factor
//                     })+
//                     _ => unreachable!()
//                 }
//             }
//         }
//         termination_automaton! {
//             // Special case of ending double-strike (in frames 9 and 10).
//             0b110 => 0b001 return 2
//             // Strike on last frame.
//             0b010 => 0b001 return 1
//             // Spare on last frame or second additional roll after strike on last frame.
//             0b001 => 0b000 return 1
//             // No rolls left, game over.
//             0b000 => return 0
//         }
//     }
//     // The game is over when all bits are set to zero..
//     fn game_over(&self) -> bool {
//         // .. except for the score overflow bit.
//         self.0 & !0b1000_0000 == 0
//     }
// }
// // Logic for the 'progression' automaton.
// impl Progression {
//     fn new() -> Self {
//         #[allow(clippy::unusual_byte_groupings)]
//             // Start with 0 pins and a fresh frame.
//             Self(0b0_0000000)
//         //     ^ ^^^^^^^
//         //     | |||||||__ Pins counter (0 to 100 then additional rolls to 120).
//         //     |__ Flag raised when a ball was already cast during the current frame.
//     }
//     // Basic reading.
//     fn total_pins(&self) -> u8 {
//         self.0 & !0b1000_0000 // Get rid of the first bit.
//     }
//     fn is_new_frame(&self) -> bool {
//         self.0 < 0b1000_0000 // Check first bit.
//     }
//     fn mark_frame_as_started(&mut self) {
//         self.0 |= 0b1000_0000; // Raise first bit.
//     }
//     // Count how many pins are still standing before next frame.
//     fn standing_pins(&self) -> u8 {
//         10 - self.total_pins() % 10
//     }
//     // Complete frame with the given number of pins and reset a fresh frame.
//     fn complete_frame(&mut self, missing_pins: u8) {
//         self.0 &= !0b1000_0000; // Remove flag.
//         self.0 += missing_pins; // Artificially knock down the remaining pins.
//     }
//     // Main usage of the automaton.
//     fn step_and_determine_roll_type(&mut self, pins: u8) -> Result<u8, Error> {
//         use Ordering::{Equal, Greater, Less};
//         let roll_type = if self.is_new_frame() {
//             match pins.cmp(&10) {
//                 Less => {
//                     // Not enough pins have been knocked down, mark this frame as already
//                     // started and give the player another chance to complete it.
//                     self.mark_frame_as_started();
//                     Ok(OPEN)
//                 }
//                 Equal => Ok(STRIKE), // Directly skip to the next frame.
//                 Greater => Err(Error::NotEnoughPinsLeft),
//             }
//         } else {
//             let standing = self.standing_pins();
//             match pins.cmp(&standing) {
//                 Less => {
//                     // The player could not complete the frame.
//                     // Mark the roll as open, artificially knock the remaining pins
//                     // and go to the next frame.
//                     self.complete_frame(standing - pins);
//                     Ok(OPEN)
//                 }
//                 Equal => {
//                     self.complete_frame(0); // The frame is already complete.
//                     Ok(SPARE)
//                 }
//                 Greater => Err(Error::NotEnoughPinsLeft),
//             }
//         };
//         // In any case, record the fallen pins.
//         self.0 += pins;
//         roll_type
//     }
// }
// // Integrate all automata into the exposed API.
// impl BowlingGame {
//     pub fn new() -> Self {
//         BowlingGame {
//             score: 0,
//             progression: Progression::new(),
//             bonus: Bonus::new(),
//         }
//     }
//     // When computing the score,
//     // take care of reporting the score overflow to the dedicated bit in `bonus`.
//     fn bump_score(&mut self, points: u8) {
//         match self.score.overflowing_add(points) {
//             (s, false) => {
//                 self.score = s;
//             }
//             (wrapped, true) => {
//                 self.bonus.set_score_overflow();
//                 self.score = wrapped + 1;
//             }
//         }
//     }
//     fn read_score(&self) -> u16 {
//         u16::from(self.score)
//             + if self.bonus.does_score_overflow() {
//             u16::from(u8::MAX)
//         } else {
//             0
//         }
//     }
//     // Main API interface.
//     pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
//         // During the main phase of the game..
//         if self.progression.total_pins() < 100 {
//             // .. determine how many times to count these pins in the score.
//             let factor = self.bonus.read_factor();
//             self.bump_score(pins * factor);
//             // .. determine the roll type and update the game state.
//             let roll_type = self.progression.step_and_determine_roll_type(pins)?;
//             self.bonus.record(roll_type);
//             // When all frames have been completed..
//             if self.progression.total_pins() == 100 {
//                 // .. switch to 'termination' phase.
//                 self.bonus
//                     .recycle_into_termination_automaton(match roll_type {
//                         OPEN => 0,   // No additional roll for an open last frame.
//                         SPARE => 1,  // One additional roll for a spare last frame.
//                         STRIKE => 2, // Two additional rolls for a strike.
//                         _ => unreachable!(),
//                     });
//             }
//             Ok(())
//         }
//         // At the end of the game..
//         else {
//             // Keep counting pins just to check for Error::NotEnoughPinsLeft.
//             let _ = self.progression.step_and_determine_roll_type(pins)?;
//             // Count these additional pins in the score.
//             match self.bonus.read_additional_factor() {
//                 0 => Err(Error::GameComplete),
//                 factor => {
//                     self.bump_score(pins * factor);
//                     Ok(())
//                 }
//             }
//         }
//     }
//     pub fn score(&self) -> Option<u16> {
//         self.bonus.game_over().then_some(self.read_score())
//     }
// }
