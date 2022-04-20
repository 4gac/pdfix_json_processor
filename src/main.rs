use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use clap::{ErrorKind, IntoApp, Parser};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct PdfixJson {
    pages: Vec<Page>,
}
#[derive(Deserialize, Debug)]
struct Page {
    page_map: PageMap,
}

#[derive(Deserialize, Debug)]
struct PageMap {
    lang: Option<String>,
    elements: Element,
    bbox: String,
}

#[derive(Deserialize, Debug)]
struct Element {
    #[serde(rename = "type")]
    tag: String,
    kids: Option<Vec<Element>>,
    text: Option<String>,
    text_style: Option<String>,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, required = true, help = "Path to JSON file")]
    input: String,
    #[clap(
        short,
        long,
        required = true,
        help = "Path where the output files will be stored"
    )]
    output: String,
    #[clap(
        short,
        long,
        required = true,
        help = "Starting page",
        parse(try_from_str)
    )]
    from_page: usize,
    #[clap(short, long, required = true, help = "Last page", parse(try_from_str))]
    to_page: usize,
    #[clap(short = 'T', long, help = "Segment text into sentences")]
    tokenize: bool,
}

fn main() {
    let args = Args::parse();
    let input = args.input;

    if !Path::new(&args.output).is_dir() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Output should be an existing directory",
        )
        .exit();
    }

    let p = Path::new(&input);
    if p.is_file() && p.extension().unwrap() == "json" {
        let file = File::open(p).expect("Could not open file");
        let pdfix_json: PdfixJson = serde_json::from_reader(file).expect("error");
        let output_sk = process_json(
            &pdfix_json,
            args.from_page,
            args.to_page,
            String::from("sk-SK"),
        );

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .truncate(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(args.output.to_owned() + p.file_stem().unwrap().to_str().unwrap() + "_sk.txt")
            .expect("Unable to open file");
        for text in output_sk {
            f.write_all(text.as_bytes()).expect("Unable to write data");
        }

        let output_en = process_json(
            &pdfix_json,
            args.from_page,
            args.to_page,
            String::from("en-GB"),
        );

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .truncate(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(args.output.to_owned() + p.file_stem().unwrap().to_str().unwrap() + "_en.txt")
            .expect("Unable to open file");
        for text in output_en {
            f.write_all(text.as_bytes()).expect("Unable to write data");
        }

        if args.tokenize {
            Command::new("python3")
                .arg("scripts/sentence_tokenizer.py")
                .arg(
                    ToOwned::to_owned(&Args::parse().output)
                        + p.file_stem().unwrap().to_str().unwrap()
                        + "_sk.txt",
                )
                .arg(
                    ToOwned::to_owned(&Args::parse().output)
                        + p.file_stem().unwrap().to_str().unwrap()
                        + "_en.txt",
                )
                .spawn()
                .expect("failed to run python script");
        }
    }
}

fn process_json(json: &PdfixJson, from_page: usize, to_page: usize, lang: String) -> Vec<String> {
    let mut to_ret = Vec::new();
    let pages = &json.pages;
    for page in pages.iter().enumerate() {
        let tag = page.1.page_map.elements.tag.as_str();
        if page.0 < from_page || page.0 > to_page {
            continue;
        }
        match tag {
            "pde_container" => {
                let kids = &page.1.page_map.elements.kids;
                for kid in kids {
                    for e in kid {
                        if e.tag == "pde_text" && e.text_style.to_owned().unwrap() == "normal" {
                            if let Some(s) = e.text.to_owned() {
                                // write to SK or EN file
                                if lang
                                    == page.1.page_map.lang.to_owned().unwrap_or_else(|| {
                                        panic!("Missing language identifier on page: {}", page.0)
                                    })
                                {
                                    to_ret.push(s + " ");
                                }
                            }
                        }
                    }
                }
            }
            _ => println!("Unknown element {}", tag),
        }
    }
    to_ret
}
