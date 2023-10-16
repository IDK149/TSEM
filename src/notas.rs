use std::collections::HashMap;

pub fn sol() -> HashMap<String, String> {
    let mut input_to_char = HashMap::new();

    input_to_char.insert("G".to_string(), "`".to_string());

    input_to_char.insert("cmp".to_string(), "-".to_string());



    // Mapeos para las notas naturales (sin bemoles ni sostenidos)
    input_to_char.insert("4A".to_string(), ")".to_string());
    input_to_char.insert("2A".to_string(), "c".to_string());
    input_to_char.insert("1A".to_string(), "o".to_string());

    // Mapeos para las notas con bemol
    input_to_char.insert("4Ab".to_string(), ")?".to_string());
    input_to_char.insert("2Ab".to_string(), "c?".to_string());
    input_to_char.insert("1Ab".to_string(), "o?".to_string());

    // Mapeos para las notas con sostenido
    input_to_char.insert("4A#".to_string(), "4lasostenido".to_string());
    input_to_char.insert("2A#".to_string(), "2lasostenido".to_string());
    input_to_char.insert("1A#".to_string(), "1lasostenido".to_string());

    input_to_char.insert("4B".to_string(), "_".to_string());
    input_to_char.insert("2B".to_string(), "v".to_string());
    input_to_char.insert("1B".to_string(), "]".to_string());

    // Mapeos para las notas con bemol con "B" en lugar de "A"
    /* nota_to_char.insert(
        "4Bb".to_string(),
        char::from_u32(0164).unwrap().to_string() + "_",
    );
    nota_to_char.insert(
        "2Bb".to_string(),
        char::from_u32(0164).unwrap().to_string() + "v",
    );
    nota_to_char.insert(
        "1Bb".to_string(),
        char::from_u32(0164).unwrap().to_string() + "]",
    ); */

    // Mapeos para las notas con sostenido con "B" en lugar de "A"
    input_to_char.insert("4B#".to_string(), "4bbemol".to_string());
    input_to_char.insert("2B#".to_string(), "2bsostenido".to_string());
    input_to_char.insert("1B#".to_string(), "1bsostenido".to_string());

    input_to_char.insert("4C".to_string(), "+".to_string());
    input_to_char.insert("2C".to_string(), "l".to_string());
    input_to_char.insert("1C".to_string(), "r".to_string());

    // Mapeos para las notas con bemol con "C" en lugar de "B"
    input_to_char.insert("4Cb".to_string(), "B+".to_string());
    input_to_char.insert("2Cb".to_string(), "Bl".to_string());
    input_to_char.insert("1Cb".to_string(), "Br".to_string());

    // Mapeos para las notas con sostenido con "C" en lugar de "B"
    input_to_char.insert("4C#".to_string(), "4csostenido".to_string());
    input_to_char.insert("2C#".to_string(), "2csostenido".to_string());
    input_to_char.insert("1C#".to_string(), "1csostenido".to_string());

    // Mapeos para las notas naturales (sin bemoles ni sostenidos) con "D" en lugar de "C"
    input_to_char.insert("4D".to_string(), "^".to_string());
    input_to_char.insert("2D".to_string(), ";".to_string());
    input_to_char.insert("1D".to_string(), "t".to_string());

    // Mapeos para las notas con bemol con "D" en lugar de "C"
    input_to_char.insert("4Db".to_string(), "N^".to_string());
    input_to_char.insert("2Db".to_string(), "N;".to_string());
    input_to_char.insert("1Db".to_string(), "Nt".to_string());

    // Mapeos para las notas con sostenido con "D" en lugar de "C"
    input_to_char.insert("4D#".to_string(), "4dsostenido".to_string());
    input_to_char.insert("2D#".to_string(), "2dsostenido".to_string());
    input_to_char.insert("1D#".to_string(), "1dsostenido".to_string());

    // Mapeos para las notas naturales (sin bemoles ni sostenidos) con "E" en lugar de "D"
    input_to_char.insert("4E".to_string(), "&".to_string());
    input_to_char.insert("2E".to_string(), "'".to_string());
    input_to_char.insert("1E".to_string(), "y".to_string());

    // Mapeos para las notas con bemol con "E" en lugar de "D"
    input_to_char.insert("4Eb".to_string(), "M&".to_string());
    input_to_char.insert("2Eb".to_string(), "M'".to_string());
    input_to_char.insert("1Eb".to_string(), "My".to_string());

    // Mapeos para las notas con sostenido con "E" en lugar de "D"
    input_to_char.insert("4E#".to_string(), "4esostenido".to_string());
    input_to_char.insert("2E#".to_string(), "2esostenido".to_string());
    input_to_char.insert("1E#".to_string(), "1esostenido".to_string());

    // Mapeos para las notas naturales (sin bemoles ni sostenidos) con "F" en lugar de "E"
    input_to_char.insert("4F".to_string(), "*".to_string());
    input_to_char.insert("2F".to_string(), "z".to_string());
    input_to_char.insert("1F".to_string(), "u".to_string());

    // Mapeos para las notas con bemol con "F" en lugar de "E"
    input_to_char.insert("4Fb".to_string(), "<*".to_string());
    input_to_char.insert("2Fb".to_string(), "<z".to_string());
    input_to_char.insert("1Fb".to_string(), "<u".to_string());

    // Mapeos para las notas con sostenido con "F" en lugar de "E"
    input_to_char.insert("4F#".to_string(), "4fsostenido".to_string());
    input_to_char.insert("2F#".to_string(), "2fsostenido".to_string());
    input_to_char.insert("1F#".to_string(), "1fsostenido".to_string());

    // Mapeos para las notas naturales (sin bemoles ni sostenidos) con "G" en lugar de "F"
    input_to_char.insert("4G".to_string(), "(".to_string());
    input_to_char.insert("2G".to_string(), "x".to_string());
    input_to_char.insert("1G".to_string(), "i".to_string());

    // Mapeos para las notas con bemol con "G" en lugar de "F"
    input_to_char.insert("4Gb".to_string(), ">(".to_string());
    input_to_char.insert("2Gb".to_string(), ">x".to_string());
    input_to_char.insert("1Gb".to_string(), ">i".to_string());

    // Mapeos para las notas con sostenido con "G" en lugar de "F"
    input_to_char.insert("4G#".to_string(), "4gsostenido".to_string());
    input_to_char.insert("2G#".to_string(), "2gsostenido".to_string());
    input_to_char.insert("1G#".to_string(), "1gsostenido".to_string());
    input_to_char
}
