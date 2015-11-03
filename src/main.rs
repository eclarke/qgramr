extern crate bio;
use bio::io::fasta;
use bio::data_structures::qgram_index;
use bio::alphabets;

fn main() {
    let mut reader = fasta::Reader::from_file("/Users/ecl/dev/swga/tests/data/ecoli_wgs.fa").ok().expect("Blah.");
    let mut rec = fasta::Record::new();
    reader.read(&mut rec).ok().expect("Error reading record");
    let seq = rec.seq();

    // let text = b"ACGGCTGAGATGAT";
    let alphabet = alphabets::dna::alphabet();
    let q = 7;
    println!("Building index over {}...", rec.id().expect("<NA>"));
    let qgram_index = qgram_index::QGramIndex::new(q, seq, &alphabet);
    println!("Done.");
    let matches = qgram_index.exact_matches(b"ATGCATGC");
    for i in matches {
        let m = i.text;
        println!("start:{}, stop:{}", m.start, m.stop);
    }
    let qgram_matches = qgram_index.qgram_matches(7);
    for i in qgram_matches {
        println!("{}", i);
    }
}
