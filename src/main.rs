#! /usr/bin/env run-cargo-script
// cargo install cargo-script
//#! /usr/bin/env rustc
// Decide on interactive vs compiling

extern crate bio;
extern crate bit_vec;

use bio::io::fasta::{Reader, Record};
use bit_vec::BitVec;

fn main() {
    let filename = "data/example.fasta";
    let reader = Reader::from_file(filename).unwrap();

    for result in reader.records() {
        let mut record = result.unwrap();
        let sequence = record.seq();

        // Allocate memory once then fill in values
        let mut bit_vec = BitVec::from_elem(record.seq().len()*4, false);

        let mut i=0;
        for base in sequence.iter() {
            match *base { // ACGT
                b'A' => bit_vec.set(i,true),
                b'C' => bit_vec.set(i+1,true),
                b'G' => bit_vec.set(i+2,true),
                b'T' => bit_vec.set(i+3,true),
                // Ambiguity codes for nucleotide degeneracy
                b'R' => { // AG
                    bit_vec.set(i,true);
                    bit_vec.set(i+2,true);
                },
                b'Y' => { // CT
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+3,true);
                },
                b'K' => { // GT
                    bit_vec.set(i+2,true);
                    bit_vec.set(i+3,true);
                },
                b'M' => { // AC
                    bit_vec.set(i,true);
                    bit_vec.set(i+1,true);
                },
                b'S' => { // GC
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+2,true);
                },
                b'W' => { // AT
                    bit_vec.set(i,true);
                    bit_vec.set(i+3,true);
                },
                b'B' => { // CGT
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+2,true);
                    bit_vec.set(i+3,true);
                },
                b'D' => { // AGT
                    bit_vec.set(i,true);
                    bit_vec.set(i+2,true);
                    bit_vec.set(i+3,true);
                },
                b'H' => { // ACT
                    bit_vec.set(i,true);
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+3,true);
                },
                b'V' => { // ACG
                    bit_vec.set(i,true);
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+2,true);
                },
                b'N' => { // ACGT
                    bit_vec.set(i,true);
                    bit_vec.set(i+1,true);
                    bit_vec.set(i+2,true);
                    bit_vec.set(i+3,true);
                },
                b'-' => { // Gap
                    bit_vec.set(i,false);
                    bit_vec.set(i+1,false);
                    bit_vec.set(i+2,false);
                    bit_vec.set(i+3,false);
                },
                _ => panic!("Invalid base: {}", base),
            }
            i+=4;
        }

        //record.set_desc(String::from_utf8_lossy(record.desc()).to_string());
        //record.set_seq(bit_vec);

        println!("{:?}", record.seq());
        println!("{:?}", bit_vec);
    }

}
