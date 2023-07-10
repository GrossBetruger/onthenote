use std::io::stdin;
use rand::distributions::uniform::SampleRange;
use rand::seq::SliceRandom;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

mod constants;
use constants::G_ASCII;


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
    fn get_ascii(&self) -> &'static str {
        match self {
            Note::A => "A",
            Note::B => "B",
            Note::C => "C",
            Note::D => "D",
            Note::E => "E",
            Note::F => "F",
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


fn notes_to_italian_test(note: &str) -> Option<&'static str> {
    println!("what is the italian name for: {}", note);
    let mut user_input = read_from_user()?;
    // stdin().read_line(&mut user_input).ok()?;

    let index = get_note_index(note)?;
    let italian = ITALIAN_NOTES[index];
    if user_input.to_ascii_lowercase().starts_with(&italian.to_ascii_lowercase()) {
        println!("correct! {} is {}", note, italian);
    }
    else {
        println!("incorrect! {} is {}", note, italian);
    }
    None
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
        if note.to_ascii_lowercase().starts_with(&answer.to_ascii_lowercase().trim()) {
            println!("correct!");
        }
        else {
            println!("incorrect! next note was {}", note);
            break;
        }
    }
}


fn american_to_italian_notes_test() {
    let american_note = NOTES.choose(&mut rand::thread_rng()).unwrap();
    notes_to_italian_test(american_note);
}

fn sheet_note_test() {
    // let rand_usize = random_usize(0, MajorScale::COUNT);
    let note: Note = Note::from_repr(6).unwrap();
    print!("what note is this? {}\n", note.get_ascii());
    let answer = read_from_user().expect("failed to read from user");
    if Note::code_to_note(&answer.trim()) == note {
        println!("correct!");
    }
    else {
        println!("incorrect! next note was {}", note.note_to_code());
    }
}

fn main() {
    sheet_note_test();
    circle_of_fifths_test();
    american_to_italian_notes_test();
}

