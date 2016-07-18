//extern crate getopts;
//use getopts::Options;
extern crate curl;
extern crate serde;
extern crate serde_json;

//use std::io;::{stdin , stdout, Write};
use curl::easy::Easy;
use serde_json::{Value};
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut opts = Options::new();

    if args.len() == 1{
        println!("usage:\n{}",args[0]);
        println!("{} string \t //To seach stuff",args[0]);
        println!("{} -i string \t //Fetch intro",args[0]);
        process::exit(0);
    }
    else {
        if args[1] == "-i"{
            if args.len() < 3{
                println!("usage:\n{}",args[0]);
                println!("{} string \t //To seach stuff",args[0]);
                println!("{} -i string \t //Fetch intro",args[0]);
                process::exit(0);
            }
            get_intro(&args[2]);
            process::exit(0);
        }
        else {
            get_search(&args[1]);
        }

    }

}

fn get_intro(article: &str) {
    //println!("intro {}", &article );
    let mut baseurl = "https://en.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exlimit=1&explaintext&exintro&titles=".to_string();
    baseurl.push_str(article);
    //println!("intro {}", baseurl);
    let mut handle = Easy::new();
    let mut data = Vec::new();
    handle.url(&baseurl).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let article_fetch = String::from_utf8(data).unwrap();
    let json: Value = serde_json::from_str(&article_fetch).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });
    let pages = json.as_object()
        .and_then(|object| object.get("query"))
        .and_then(|query| query.as_object())
        .and_then(|object| object.get("pages"))
        .and_then(|pages| pages.as_object())
        .unwrap_or_else(|| {
            panic!("Failed to get 'pages' value from json");
        });
        for (rel, page) in pages.iter() {
            let title = page.find("title")
                .and_then(|value| value.as_string())
                .unwrap_or_else(|| {
                    panic!("Failed to get 'extract' value from within 'pages'");
                });
            let intro = page.find("extract")
                .and_then(|value| value.as_string())
                .unwrap_or_else(|| {
                    panic!("Failed to get 'extract' value from within 'pages'");
                });

        println!("{}:",title);
        println!("{}", intro);
    }

}

fn get_search(article: &str) {
    //println!("search {}", &article );
    let mut baseurl = "https://en.wikipedia.org/w/api.php?action=opensearch&limit=5&namespace=0&format=json&search=".to_string();
    baseurl.push_str(article);
    //println!("search {}", baseurl );
    let mut handle = Easy::new();
    let mut data = Vec::new();

    handle.url(&baseurl).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let article_fetch = String::from_utf8(data).unwrap();
    let json: Value = serde_json::from_str(&article_fetch).unwrap_or_else(|e| {
        panic!("Failed to parse json; error is {}", e);
    });
    let search_data = json.as_array();
    //for search_data.iter()
    //let mut link = Vec::new();
    //let mut desc = Vec::new();
    //println!("{:?}",search_data);
    for sd in search_data.iter() {
        println!("{:?}", sd[0]);
        println!("{:?}", sd[1]);
        println!("{:?}", sd[2]);
        println!("{:?}", sd[3]);

        /*let data : Value = serde_json::from_str(sd[0]).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });*/
        //let data1 = sd[1].as_array();
        //let data2 = sd[2].as_array();
        //let data3 = sd[3].as_array();
        /*for d1 in data1.iter(){
            println!("{}",d1[0]);
            println!("1");
        }*/
        //println!("{}",&names);
        //println!("{:?}", sd[2]);
        //println!("{:?}", sd[3]);
    }
    //println!("{:?}", search_data);
}
