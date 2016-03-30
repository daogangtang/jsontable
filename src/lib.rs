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

mod formatter;

pub use formatter::beatufly;
