// Del usuario vamos a recibir una serie de instrucciones que tenemos que pasar a cada una de las letras que simboliza dentro del alfabeto que tenemos
// Comandos:
// !size(x, y)
// !title(String)
// !font
use printpdf::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

fn export(text: String) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(200.0), Mm(200.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_external_font(File::open("../../../Downloads/lassus/LASSUS.TTF").unwrap())
        .unwrap();
    // returnfont('A');
    current_layer.use_text(text, 20 as f32, Mm(10.0), Mm(100.0), &font);
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
        section.push(lines.unwrap() + "~~")
    }
    Ok(section.concat())
}
fn main() {
    let input = input().unwrap();
    println!("{input}\n");
    // parsear el vector:
    if input.contains("!") {
        println!("config")
    } else {
        println!("Not config")
    }
    // export(input);
}
 
