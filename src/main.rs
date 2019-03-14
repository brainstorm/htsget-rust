extern crate rust_htslib;

fn main() {
    println!("Hello world");

    use rust_htslib::bam;
    use rust_htslib::prelude::*;

    let mut bam = bam::Reader::from_path(&"test/test.bam").unwrap();
    let header = bam::Header::from_template(bam.header());
    let mut out = bam::Writer::from_path(&"test/out.bam", &header).unwrap();

    // copy reverse reads to new BAM file
    for r in bam.records() {
        let record = r.unwrap();
        if record.is_reverse() {
            out.write(&record).unwrap();
        }
    }
}