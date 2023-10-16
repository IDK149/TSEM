extern crate regex;
use printpdf::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
mod notas;

fn export(text: String, config: (u32, (u32, u32))) {
    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Mm(config.1 .0 as f32),
        Mm(config.1 .1 as f32),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_external_font(File::open("../../../Downloads/lassus/LASSUS.TTF").unwrap())
        .unwrap();
    current_layer.use_text(text, config.0 as f32, Mm(10.0), Mm(100.0), &font);
    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}

fn input() -> std::io::Result<String> {
    let archivo = File::open("Input.txt")?;
    let buf_reader = BufReader::new(archivo);
    let mut section = Vec::new();
    for lines in buf_reader.lines() {
        section.push(lines.unwrap())
    }
    Ok(section.concat())
}

fn search(parametro: &str, input: String) -> Vec<String> {
    let re = Regex::new(&parametro).unwrap();
    let captures = re.captures_iter(&input);
    let mut strings = Vec::new();
    for i in captures {
        strings.push(i.get(1).unwrap().as_str().to_owned())
    }
    return strings;
}
// FIXME
// Considero que esto no está bien optimizado
fn config() -> (u32, (u32, u32)) {
    let psize = search(r"(?:psize)\(([^)]+)\)", input().unwrap());
    let fsize = search(r"(?:fsize)\(([^)]+)\)", input().unwrap());
    let mut psize_vec = Vec::new();
    let mut fsize_vec = Vec::new();
    for i in psize {
        let hola: Vec<u32> = i.split(',').filter_map(|s| s.trim().parse().ok()).collect();
        for e in hola {
            psize_vec.push(e);
        }
    }
    for i in fsize {
        fsize_vec.push(i.parse().unwrap())
    }
    let psize_vec = psize_vec;
    (fsize_vec[0], (psize_vec[0], psize_vec[1]))
}


fn replace_notes_font(search: Vec<String>) -> String {
    let mut union = Vec::new();
    for i in search {
        if let Some(replacement) = notas::sol().get(&i) {
            union.push(replacement.to_owned())
        } else {
            union.push(" ".to_string())
        }
    }
    return union.concat();
}

fn compas() -> std::io::Result<String> {
    let notas = search(r"(?:nota)\(([^)]+)\)", input().unwrap());
    let compas: f32 = search(r"(?:compas)\(([^)]+)\)", input().unwrap())
        .concat()
        .parse()
        .unwrap();
    println!("{}", compas);
    println!("{:?}", notas);
    let mut result = 0.0;
    for i in notas {
        let number: f32 = i.chars().next().unwrap().to_string().parse().unwrap();
        if result >= compas {
            return Ok("-".to_string());
        } else {
            result += 1.0 / number;
            continue;
        }
    }
    Ok("".to_string())
}

fn main() {
    println!("{}", compas().unwrap())
}
