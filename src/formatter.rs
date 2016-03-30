
use std::io::{Read};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use hyper::Client;


pub fn beatufly(url: &str) -> String {
    println!("send to: {}", url);

    //let url = "http://120.55.184.150:8585/datamin/top_fav_artist";
    let client = Client::new();

    // do request
    let mut cres = client.get(url)
    .send().unwrap();

    //println!("{:?}", cres);

    let mut body = String::new();
    cres.read_to_string(&mut body).unwrap();
    //println!("ret value: {}", body);

    let jsonstr = &body;

    //let jsonstr = "{\"success\": true, \"data\":[{\"foo\": 13, \"bar\": \"baz\"}, {\"foo\": 15, \"bar\": \"bazzzz\"}]}";

    let json_data: Value = serde_json::from_str(jsonstr).unwrap();
    //println!("{:?}", json_data);

    //println!("object? {}", json_data.is_object());

    let json_obj = json_data.as_object().unwrap();
    let data = json_obj.get("data").unwrap();
    let data_obj = data.as_array().unwrap();

    let mut collector: HashMap<&str, Vec<String>> = HashMap::new();

    for item in data_obj {
        let item_obj = item.as_object().unwrap();
        for (key, val) in item_obj.iter() {
            //println!("{}", key);
            //println!("{:?}", val);
            // 先遍历出表头名，然后创建数组数据容器
            let entry_vec = collector.entry(key).or_insert(vec![]);

            if val.is_string() {
                // 向数组中添加数据
                entry_vec.push(val.as_string().unwrap().to_string());
            }
            else if val.is_u64() {
                entry_vec.push(val.as_u64().unwrap().to_string());
            }
            else if val.is_i64() {
                entry_vec.push(val.as_i64().unwrap().to_string());
            }
            else if val.is_f64() {
                entry_vec.push(val.as_f64().unwrap().to_string());
            }
            else {
                // other type
            }

        }
    }

    //println!("collector is: {:#?}", collector);


    // 转换成一个二维矩阵
    let mut key_arr: Vec<String> = vec![];
    let mut val_arr: Vec<Vec<String>> = vec![];
    for (key, val) in &collector {
        key_arr.push(key.to_string());
        val_arr.push(val.clone());
    }

    let mut arr2d: Vec<Vec<String>> = vec![];
    for i in 0..(val_arr[0].len()) {
        let mut row: Vec<String> = vec![];
        for j in 0..val_arr.len() {
            let col = &val_arr[j];
            if i < col.len() {
                row.push(col[i].clone());
            }
        }

        arr2d.push(row);
    }

    //println!("{:?}", arr2d);

    // 下面是用prettytable美化输出
    let mut table = Table::new();

    for rowd in &arr2d {
        let mut row = Row::empty();
        for item in rowd {
            row.add_cell(Cell::new(item));
        }
        table.add_row(row);
    }

    let mut titles = Row::empty();
    for v in &key_arr {
        titles.add_cell(Cell::new(v));
    }
    table.set_titles(titles);

    //table.printstd();

    //let mut out = String::new();
    //table.print(&mut out);


    table.to_string()

}
