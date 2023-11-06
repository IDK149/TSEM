extern crate regex;
use printpdf::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
mod notas;

// Todo: - Hacer una función de alineación a la derecha, a la izquierda y al centro
// otra función para pasar de la medida de la página a la cantidad de caracteres
// cambiar la bold de los subtitulos por regular

fn export(text: String, top: (String, String, String)) {
    let (doc, page1, layer1) = PdfDocument::new("", Mm(210 as f32), Mm(297 as f32), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let default_font = doc
        .add_external_font(File::open("../Font/DejaVuSansMNerdFont-Bold.ttf").unwrap())
        .unwrap();
    let music_font = doc
        .add_external_font(File::open("../Font/LassusV2.otf").unwrap())
        .unwrap();
    let characteres_music = 46;
    let characteres_default = 70;
    let characteres_default_minor = 86;
    let sample = separar(&text, characteres_music as f32 - 4.0);
    let mut linebreak = 10.0;

    for i in sample {
        current_layer.use_text(i, 60.0, Mm(10.0), Mm(255.0 - linebreak), &music_font);
        linebreak += 20.0;
    }
    // Title
    current_layer.use_text(
        &top.0,
        15.0,
        Mm(
            (210 as f32 * (characteres_default as f32 - top.0.chars().count() as f32) / 2.0)
                / characteres_default as f32,
        ),
        Mm(297.0 - 10.0),
        &default_font,
    );
    // subTitle
    current_layer.use_text(
        &top.1,
        12.0,
        Mm((210.0 / characteres_default_minor as f32)
            * (characteres_default_minor as f32 - top.1.len() as f32)
            - 10.0),
        Mm(297.0 - 25.0),
        &default_font,
    );
    current_layer.use_text(
        &top.2,
        12.0,
        Mm((210.0 / characteres_default_minor as f32)
            * (characteres_default_minor as f32 - top.2.len() as f32)
            - 10.0),
        Mm(297.0 - 32.0),
        &default_font,
    );

    doc.save(&mut BufWriter::new(File::create("Partitura.pdf").unwrap()))
        .unwrap();
}

fn config() -> (String, String, String) {
    let title = regex_search(r"(?:title)\(([^)]+)\)", input_file().unwrap());
    let subtitle = regex_search(r"(?:sub)\(([^)]+)\)", input_file().unwrap());
    let author = regex_search(r"(?:autor)\(([^)]+)\)", input_file().unwrap());
    (title.concat(), subtitle.concat(), author.concat())
}

fn regex_search(parametro: &str, input: String) -> Vec<String> {
    let re = Regex::new(&parametro).unwrap();
    let captures = re.captures_iter(&input);
    let mut strings = Vec::new();
    for i in captures {
        strings.push(i.get(1).unwrap().as_str().to_owned())
    }
    return strings;
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

// Info: vec[0] = clave
fn search_times(texto: &str) -> Vec<Vec<Vec<String>>> {
    let re = Regex::new(r"clave\(([^)]+)\)\[([^\]]*)\]").unwrap();
    let mut fullvector = Vec::new();

    for captures in re.captures_iter(texto) {
        let sub_re = Regex::new(r"\((\d+/\d+)\)\{([^}]*)\}").unwrap();
        let mut container_notes = Vec::new();
        if let Some(captura) = captures.get(1) {
            container_notes.push(vec![captura.as_str().to_string()]);
        }

        for subcaptures in sub_re.captures_iter(&captures[2]) {
            let tiempo = &subcaptures[1];
            let contenido = &subcaptures[2];
            let re_notas = Regex::new(r"(?:nota)\(([^)]+)\)").unwrap();
            let mut subvector = Vec::new();

            subvector.push(tiempo.to_string());

            for nota in re_notas.captures_iter(contenido) {
                let contenido_nota = &nota[1];
                subvector.push(contenido_nota.to_string());
            }

            container_notes.push(subvector);
        }

        fullvector.push(container_notes);
    }

    fullvector
}

fn times_logic(e: Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<String>>> {
    let mut returning_vector = Vec::new();
    for i in e {
        let mut container_vectors = Vec::new();
        for (index, e) in i.iter().enumerate() {
            if index == 0 {
                // returning_vector.push(vec![e.clone()])
            }
            let mut vectors = Vec::new();
            let mut result = 0.0;
            for (index, elements) in e.iter().enumerate() {
                if index == 0 {
                    vectors.push(elements.to_string())
                } else {
                    let numero: f32 = elements
                        .chars()
                        .next()
                        .unwrap()
                        .to_string()
                        .parse()
                        .unwrap_or(0.0);
                    let tempo = relative_bar(&e[0]);

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
            container_vectors.push(vectors)
        }
        returning_vector.push(container_vectors)
    }
    returning_vector
}

fn replace_to_lassusfont(search: Vec<Vec<Vec<String>>>) -> String {
    let mut union = Vec::new();
    for i in search {
        match i[0][0].as_str() {
            "G" => {
                for e in i {
                    for j in e {
                        if let Some(replacement) = notas::sol().get(&j.to_string()) {
                            union.push(replacement.to_owned())
                        } else {
                            union.push(" ".to_string())
                        }
                    }
                }
            }
            "F" => {
                for e in i {
                    for j in e {
                        if let Some(replacement) = notas::fa().get(&j.to_string()) {
                            union.push(replacement.to_owned())
                        } else {
                            union.push(" ".to_string())
                        }
                    }
                }
            }
            "C1" => {
                for e in i {
                    for j in e {
                        if let Some(replacement) = notas::do1().get(&j.to_string()) {
                            union.push(replacement.to_owned())
                        } else {
                            union.push(" ".to_string())
                        }
                    }
                }
            }
            "C2" => {
                for e in i {
                    for j in e {
                        if let Some(replacement) = notas::do2().get(&j.to_string()) {
                            union.push(replacement.to_owned())
                        } else {
                            union.push(" ".to_string())
                        }
                    }
                }
            }
            _ => println!(""),
        }
    }
    union.concat()
}
fn separar(texto: &str, tamaño: f32) -> Vec<&str> {
    let re = Regex::new(&format!(r".{{1,{}}}(.$)?", tamaño)).unwrap();
    re.find_iter(texto).map(|mat| mat.as_str()).collect()
}

fn main() {
    let searchtimes = search_times(&input_file().unwrap());
    let hola = times_logic(searchtimes);
    export(replace_to_lassusfont(hola), config())
}
