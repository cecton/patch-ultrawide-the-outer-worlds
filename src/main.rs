use std::fs;
use std::io;
use std::io::Write;

static FILE_PATH: &str = "IndianaEpicGameStore-Win64-Shipping.exe";
static BACKUP: &str = "IndianaEpicGameStore-Win64-Shipping.bak";

#[rustfmt::skip]
static REPLACEMENTS: &[Replacement] = &[
    Replacement {
        expected_offset: 0x1c5c978,
        opcodes: &[
            &[0x83, 0xe0, 0x01],
            &[0x31, 0x41, 0x30],
        ],
        with: &[], // NOTE: no replacement, just skip the first occurrence
    },
    Replacement {
        expected_offset: 0x1c5d9d8,
        opcodes: &[
            &[0x83, 0xe0, 0x01],
            &[0x31, 0x41, 0x30],
        ],
        with: &[
            &[0x83, 0xe0, 0x00],
        ],
    },
    Replacement {
        expected_offset: 0x2861824,
        opcodes: &[
            &[0x35, 0xfa, 0x0e, 0x3c, 0xa1],
        ],
        with: &[
            &[0x35, 0x20, 0x2a, 0x3c, 0xa1],
        ],
    },
];

struct Replacement {
    expected_offset: usize,
    opcodes: &'static [&'static [u8]],
    with: &'static [&'static [u8]],
}

impl Replacement {
    fn replace(&self, buffer: &mut [u8], start: usize) -> Option<usize> {
        let len = self.opcodes.iter().fold(0, |acc, x| acc + x.len());
        let seq: Vec<u8> = self.opcodes.iter().cloned().flatten().cloned().collect();
        let repl_len = self.with.iter().fold(0, |acc, x| acc + x.len());
        let repl_seq: Vec<u8> = self.with.iter().cloned().flatten().cloned().collect();

        for i in start..(buffer.len() - len) {
            if &buffer[i..(i + len)] == seq.as_slice() {
                if i != self.expected_offset {
                    eprintln!(
                        "WARNING: Unexpected offset for replacement: 0x{:x} != 0x{:x}",
                        i, self.expected_offset
                    );
                }
                buffer[i..(i + repl_len)].clone_from_slice(repl_seq.as_slice());
                return Some(i + len);
            }
        }

        None
    }

    fn is_empty(&self) -> bool {
        self.with.is_empty()
    }
}
fn main() {
    println!("File path: {}", FILE_PATH);
    println!("Backup path: {}", BACKUP);
    let mut buffer = fs::read(FILE_PATH).expect("could not open file");
    let mut i = 0x600;
    let mut failed = false;
    for repl in REPLACEMENTS.iter() {
        if let Some(next_pos) = repl.replace(&mut buffer, i) {
            i = next_pos;
            if !repl.is_empty() {
                println!("Replacement succeeded.");
            }
        } else {
            failed = true;
            println!("Replacement failed.");
        }
    }
    let mut answer = String::new();
    if failed {
        println!("ERROR: Some patches have failed!");
        io::stdin()
            .read_line(&mut answer)
            .expect("cannot read STDIN");
        return;
    }
    print!("Proceed? [y/n] ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut answer)
        .expect("cannot read STDIN");
    if answer.trim() == "y" {
        fs::copy(FILE_PATH, &BACKUP).expect("could not backup file");
        fs::write(FILE_PATH, buffer).expect("could not overwrite file");
        println!("Succeeded.");
    } else {
        println!("Aborted.");
    }
}
