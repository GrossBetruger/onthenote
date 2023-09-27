use std::io::stdin;
use rand::distributions::uniform::SampleRange;
use rand::seq::SliceRandom;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

mod constants;
use constants::G_ASCII;
use crate::constants::{A_ASCII, B_ASCII, C_ASCII, D_ASCII, E_ASCII, F_ASCII};
use colored::Colorize;


const NOTES: [&'static str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
const ITALIAN_NOTES : [&'static str; 7] = ["La", "Si", "Do", "Re", "Mi", "Fa", "Sol"];
// const SHARPS: [&'static str; 5] = ["A#", "C#", "D#", "F#", "G#"];
// const FLATS: [&'static str; 5] = ["Bb", "Db", "Eb", "Gb", "Ab"];

const A_MAJOR_SCALE: [&'static str; 7] = ["A", "B", "C#", "D", "E", "F#", "G#"];
const B_MAJOR_SCALE: [&'static str; 7] = ["B", "C#", "D#", "E", "F#", "G#", "A#"];
const C_MAJOR_SCALE: [&'static str; 7] = ["C", "D", "E", "F", "G", "A", "B"];
const D_MAJOR_SCALE: [&'static str; 7] = ["D", "E", "F#", "G", "A", "B", "C#"];
const E_MAJOR_SCALE: [&'static str; 7] = ["E", "F#", "G#", "A", "B", "C#", "D#"];
const F_MAJOR_SCALE: [&'static str; 7] = ["F", "G", "A", "Bb", "C", "D", "E"];
const G_MAJOR_SCALE: [&'static str; 7] = ["G", "A", "B", "C", "D", "E", "F#"];

const A_MINOR_SCALE: [&'static str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
const B_MINOR_SCALE: [&'static str; 7] = ["B", "C#", "D", "E", "F#", "G", "A"];
const C_MINOR_SCALE: [&'static str; 7] = ["C", "D", "Eb", "F", "G", "Ab", "Bb"];
const D_MINOR_SCALE: [&'static str; 7] = ["D", "E", "F", "G", "A", "Bb", "C"];
const E_MINOR_SCALE: [&'static str; 7] = ["E", "F#", "G", "A", "B", "C", "D"];
const F_MINOR_SCALE: [&'static str; 7] = ["F", "G", "Ab", "Bb", "C", "Db", "Eb"];
const G_MINOR_SCALE: [&'static str; 7] = ["G", "A", "Bb", "C", "D", "Eb", "F"];

const MAJOR_SCALE_INTERVALS: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];
const MINOR_SCALE_INTERVALS: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];

// circle of fifths trick
const MAJOR_SCALE_SHARPS: [&'static str; 7] = ["F#", "C#", "G#", "D#", "A#", "E#", "B#"];
const MAJOR_SCALE_FLATS: [&'static str; 7] = ["Bb", "Eb", "Ab", "Db", "Gb", "Cb", "Fb"];

#[derive(FromRepr, Debug, PartialEq, EnumCount)]
enum ChordFunction {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

#[derive(FromRepr, Debug, PartialEq, EnumCount)]
enum MajorScale {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl MajorScale {
    fn get_major_scale(&self) -> &'static [&'static str; 7] {
        match self {
            MajorScale::A => &A_MAJOR_SCALE,
            MajorScale::B => &B_MAJOR_SCALE,
            MajorScale::C => &C_MAJOR_SCALE,
            MajorScale::D => &D_MAJOR_SCALE,
            MajorScale::E => &E_MAJOR_SCALE,
            MajorScale::F => &F_MAJOR_SCALE,
            MajorScale::G => &G_MAJOR_SCALE,
        }
    }
}


#[derive(FromRepr, Debug, PartialEq, EnumCount)]
enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Note {
    fn get_ascii_art(&self) -> &'static str {
        match self {
            Note::A => A_ASCII,
            Note::B => B_ASCII,
            Note::C => C_ASCII,
            Note::D => D_ASCII,
            Note::E => E_ASCII,
            Note::F => F_ASCII,
            Note::G => G_ASCII,
        }
    }

    fn code_to_note(code: &str) -> Note {
        match code {
            "A" => Note::A,
            "B" => Note::B,
            "C" => Note::C,
            "D" => Note::D,
            "E" => Note::E,
            "F" => Note::F,
            "G" => Note::G,
            _ => panic!("invalid note code"),
        }
    }

    fn note_to_code(&self) -> &'static str {
        match self {
            Note::A => "A",
            Note::B => "B",
            Note::C => "C",
            Note::D => "D",
            Note::E => "E",
            Note::F => "F",
            Note::G => "G",
        }
    }
}

fn get_note_index(note: &str) -> Option<usize> {
    for (i, n) in NOTES.iter().enumerate() {
        if *n == note {
            return Some(i);
        }
    }
    None
}


fn read_from_user() -> Option<String> {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).ok()?;
    Some(user_input)
}

fn correct_output(output: &str) {
    println!("{}", output.green())
}

fn incorrect_output(output: &str) {
    println!("{}", output.red())
}


fn notes_to_italian_test(note: &str) {
    println!("what is the italian name for: {}", note);
    let ref mut user_input = read_from_user().expect("failed to read from user");

    let index = get_note_index(note).expect(format!("invalid note, {}", note).as_str());
    let italian = ITALIAN_NOTES[index];
    if user_input.to_ascii_lowercase().starts_with(&italian.to_ascii_lowercase()) {
        correct_output(format!("correct! {} is {}", note, italian).as_str());
    }
    else {
        incorrect_output(format!("incorrect! {} is {}", note, italian).as_str());
    }
}


fn random_usize(min: usize, max: usize) -> usize {
    let range = min..max;
    range.sample_single(&mut rand::thread_rng())
}


fn circle_of_fifths_test() {
    let rand_usize = random_usize(0, MajorScale::COUNT);
    let scale: MajorScale = MajorScale::from_repr(rand_usize).unwrap();
    let scale_notes = *scale.get_major_scale();
    println!("what are the notes of major scale: {:?}", scale);
    for note in scale_notes.iter() {
        let answer = read_from_user().expect("failed to read from user");
        if answer.is_empty() {
            println!("skipping note..., the answer was {}", note);
            continue;
        }
        if note.to_ascii_lowercase() == answer.to_ascii_lowercase().trim() {
            correct_output("correct!");
        }
        else {
            incorrect_output(format!("incorrect! next note was {}", note).as_str());
            break;
        }
    }
}


fn american_to_italian_notes_test() {
    let american_note = NOTES.choose(&mut rand::thread_rng()).unwrap();
    notes_to_italian_test(american_note);
}


fn sheet_note_test() {
    let rand_usize = random_usize(0, MajorScale::COUNT);
    let note: Note = Note::from_repr(rand_usize).unwrap();
    print!("what note is this? {}\n", note.get_ascii_art());
    let answer = read_from_user().expect("failed to read from user");
    if Note::code_to_note(&answer.trim().to_uppercase()) == note {
        correct_output("correct!");
    }
    else {
        incorrect_output(format!("incorrect! note was {}", note.note_to_code()).as_str());
    }
}

fn chord_function_test() {
    let rand_usize = random_usize(0, MajorScale::COUNT);
    let scale: MajorScale = MajorScale::from_repr(rand_usize).unwrap();
    let scale_notes = *scale.get_major_scale();
    let rand_note = scale_notes.choose(&mut rand::thread_rng()).unwrap();
    // let random_function = [ChordFunction::I, ChordFunction::II, ChordFunction::III, ChordFunction::IV, ChordFunction::V, ChordFunction::VI, ChordFunction::VII].choose(&mut rand::thread_rng()).unwrap();
    let function_index = random_usize(0, ChordFunction::COUNT);
    let root = scale_notes.get(function_index).expect("failed to get root note");
    let function = ChordFunction::from_repr(function_index).expect("failed to get chord function");

    let chord_type = match function {
        ChordFunction::I => { "maj7" },
        ChordFunction::II => { "m7" },
        ChordFunction::III => { "m7" },
        ChordFunction::IV => { "maj7" },
        ChordFunction::V => { "7" },
        ChordFunction::VI => { "m7" },
        ChordFunction::VII => { "m7b5" },
    };

    // concate root and chord type to var chord
    let chord = format!("{}{}", root, chord_type);
    println!("what is the {:?} chord of major scale: {:?}", function, scale);

    let answer = read_from_user().expect("failed to read from user");
    if answer.trim().to_ascii_lowercase() == chord.to_ascii_lowercase() {
        correct_output("correct!");
    } else {
        incorrect_output(format!("incorrect! chord was {}", chord).as_str());
    }
}


fn choose_game() {
    println!("\nchoose a game:\n");
    println!("1. sheet note");
    println!("2. circle of fifths");
    println!("3. american to italian notes");
    println!("4. chord function");
    println!();
    let ref mut user_input = String::new();
    stdin().read_line(user_input).expect("failed to read input");
    let game = user_input.trim().parse::<u8>().unwrap();
    match game {
        1 => sheet_note_test(),
        2 => circle_of_fifths_test(),
        3 => american_to_italian_notes_test(),
        4 => chord_function_test(),
        _ => println!("invalid game"),
    }
}


fn main() {
    loop {
        choose_game();
    }
}

