//! 设计本功能的基本思路
//! 把 json 数据取出来后，先遍历其 Data 域数组，把字段建出来，然后根据字段
//! 设计插槽。就是用一个hashmap
//!


#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate prettytable;
extern crate hyper;
extern crate clap;

use clap::{Arg, App};


mod formatter;


fn main () {
    let matches = App::new("jsontable")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daogang Tang <miketang84@outlook.com>")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true)
            .required(true)
            .help("url to get json return."))
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let output = formatter::beatufly(url);
    println!("{}", output);
}
