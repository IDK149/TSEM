extern crate regex;
use printpdf::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
mod notas;

fn export(text: String) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(200.0), Mm(200.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_external_font(File::open("../../../Downloads/lassus/LASSUS.TTF").unwrap())
        .unwrap();
    current_layer.use_text(text, 50 as f32, Mm(10.0), Mm(100.0), &font);
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

fn serch(parametro: String, input: String) -> Vec<String> {
    let re = Regex::new(&parametro).unwrap();
    let captures = re.captures_iter(&input);
    let mut strings = Vec::new();
    for i in captures {
        strings.push(i.get(1).unwrap().as_str().to_owned())
    }
    return strings;
}

fn notes() -> String{
    let notas = serch(r"(?:nota|clave)\(([^)]+)\)".to_string(), input().unwrap());
    let mut union = Vec::new();
    for i in notas {
        let notas = notas::sol();
        if let Some(replacement) = notas.get(&i) {
            union.push(replacement.to_owned())
        } else {
            union.push(" ".to_string())
        }
    }
    return union.concat()
}

fn main() {
    export(notes());
}
