extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashSet;

use clap::{App, Arg};
use hyper::rt::{self, Future};

mod s3;

fn main() {
    let matches = App::new("XBeam Engineering Challenge")
        .arg(Arg::with_name("file1")
            .required(true)
            .index(1))
        .arg(Arg::with_name("file2")
            .required(true)
            .index(2))
        .get_matches();

    let file1 = matches.value_of("file1").unwrap();
    let file1 = String::from(file1);
    let file2 = matches.value_of("file2").unwrap();
    let file2 = String::from(file2);

    let client = s3::S3Client::new();
    let futures = vec![file1, file2].into_iter().map(move |file| {
        let client = client.clone();
        client.fetch_file(&file.trim())
    });
    let f = futures::future::join_all(futures)
        .map(|results| {
            match results.as_slice() {
                [first, second] => {
                    let first = first.domain_set();
                    let second = second.domain_set();
                    let overlap: HashSet<_> = first.intersection(&second).collect();
                    println!("{} {} {}", first.len(), second.len(), overlap.len())
                },
                _ => unreachable!(),
            }
        }).map_err(|err| println!("{:?}", err));

    rt::run(f);
}
