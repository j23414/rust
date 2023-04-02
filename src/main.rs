#! /usr/bin/env run-cargo-script
// cargo install cargo-script
//#! /usr/bin/env rustc
// Decide on interactive vs compiling

extern crate bio;
extern crate bit_vec;

use bio::io::fasta;
use bit_vec::BitVec;

fn main() {
    let filename = "data/example.fasta";
    let reader = fasta::Reader::from_file(filename).unwrap();

    for result in reader.records() {
        let mut record = result.unwrap();
        let sequence = record.seq();
        let mut bit_vec = BitVec::new();

        for base in sequence.iter() {
            match *base {
                b'A' => bit_vec.extend_from_slice(&[true, false, false, false]),
                b'C' => bit_vec.extend_from_slice(&[false, true, false, false]),
                b'G' => bit_vec.extend_from_slice(&[false, false, true, false]),
                b'T' => bit_vec.extend_from_slice(&[false, false, false, true]),
                _ => panic!("Invalid base: {}", base),
            }
        }

        record.set_desc(String::from_utf8_lossy(record.desc()).to_string());
        record.set_seq(bit_vec);

        println!("{:?}", record.seq());
    }

}
