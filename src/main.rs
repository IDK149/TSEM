extern crate regex;
use printpdf::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
mod notas;

fn export(text: String, configuration: (u32, (u32, u32), String)) {
    let (doc, page1, layer1) = PdfDocument::new(
        "",
        Mm(configuration.1 .0 as f32),
        Mm(configuration.1 .1 as f32),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let default_font = doc
        .add_external_font(
            // Warn: Change this to relative path
            File::open("../../../Downloads/DejaVuSansMono/DejaVuSansMNerdFont-Bold.ttf").unwrap(),
        )
        .unwrap();
    let music_font = doc
        .add_external_font(File::open("../../../Downloads/lassus/LassusV2.otf").unwrap())
        .unwrap();

    current_layer.use_text(
        configuration.2,
        12.0,
        Mm(configuration.1 .0 as f32 / 3.0),
        Mm(configuration.1 .1 as f32 - 10.0),
        &default_font,
    );
    current_layer.use_text(
        text,
        configuration.0 as f32,
        Mm(10.0),
        Mm(100.0),
        &music_font,
    );

    doc.save(&mut BufWriter::new(File::create("Partitura.pdf").unwrap()))
        .unwrap();
}


// Perf: I don't like the config function and the regex_search function
fn regex_search(parametro: &str, input: String) -> Vec<String> {
    let re = Regex::new(&parametro).unwrap();
    let captures = re.captures_iter(&input);
    let mut strings = Vec::new();
    for i in captures {
        strings.push(i.get(1).unwrap().as_str().to_owned())
    }
    return strings;
}

fn config() -> (u32, (u32, u32), String) {
    let psize = regex_search(r"(?:psize)\(([^)]+)\)", input_file().unwrap());
    let fsize = regex_search(r"(?:fsize)\(([^)]+)\)", input_file().unwrap());
    let title = regex_search(r"(?:title)\(([^)]+)\)", input_file().unwrap());
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
    (fsize_vec[0], (psize_vec[0], psize_vec[1]), title.concat())
}

fn replace_to_lassusfont(search: Vec<Vec<String>>) -> String {
    let mut union = Vec::new();
    for i in search {
        for e in i {
            if let Some(replacement) = notas::sol().get(&e) {
                union.push(replacement.to_owned())
            } else {
                union.push(" ".to_string())
            }
        }
    }
    println!("{}", union.concat());
    return union.concat();
}

fn relative_bar(input: &str) -> f32 {
    match input {
        "2/4" => 0.5,
        "3/4" => 0.75,
        "4/4" => 1.0,
        "3/2" => 1.5,
        _ => 0.0,
    }
}

fn input_file() -> std::io::Result<String> {
    let archivo = File::open("Input.txt")?;
    let buf_reader = BufReader::new(archivo);
    let mut section = Vec::new();
    for lines in buf_reader.lines() {
        section.push(lines.unwrap())
    }
    Ok(section.concat())
}

fn search_times(texto: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"\((\d+/\d+)\)\{([^}]*)\}").unwrap();
    let mut container_notes = Vec::new();
    let mut subvector = Vec::new();

    for cap in re.captures_iter(texto) {
        let tiempo = &cap[1];
        let contenido = &cap[2];
        let re_notas = Regex::new(r"(?:nota|clave)\(([^)]+)\)").unwrap();

        if !subvector.is_empty() {
            container_notes.push(subvector.clone());
            subvector.clear();
        }

        subvector.push(tiempo.to_string());

        for nota in re_notas.captures_iter(contenido) {
            let contenido_nota = &nota[1];
            subvector.push(contenido_nota.to_string());
        }
    }

    if !subvector.is_empty() {
        container_notes.push(subvector);
    }

    println!("{:?}", container_notes);
    container_notes
}

fn times_logic(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut container_vectors = Vec::new();

    for item in input {
        let mut vectors = Vec::new();
        let mut result = 0.0;

        for (index, elements) in item.iter().enumerate() {
            if index == 0 {
                if index + 1 < item.len() {
                    vectors.push(item[index + 1].to_string());
                }
            } else if index == 1 {
                vectors.push(item[0].to_string());
            } else {
                let numero: f32 = elements
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap_or(0.0);
                let tempo = relative_bar(&item[0]);

                if (1.0 / numero) + result == tempo {
                    vectors.push(elements.to_string());
                    vectors.push("-".to_string());
                    result = 0.0;
                } else {
                    result += 1.0 / numero;
                    vectors.push(elements.to_string());
                }
            }
        }

        container_vectors.push(vectors);
    }

    println!("{:?}", container_vectors);
    container_vectors
}

fn main() {
    let input = search_times(&input_file().unwrap());
    let hola = replace_to_lassusfont(times_logic(input));
    export(hola, config())
}
