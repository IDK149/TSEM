use std::collections::HashMap;

pub fn sol() -> HashMap<String, String> {
    let mut input_to_char = HashMap::new();

    // Claves
    input_to_char.insert("G".to_string(), "`".to_string());
    input_to_char.insert("F".to_string(), "1".to_string());
    input_to_char.insert("C1".to_string(), "2".to_string());
    input_to_char.insert("C2".to_string(), "3".to_string());

    //Silents
    input_to_char.insert("8s".to_string(), "7".to_string());
    input_to_char.insert("4s".to_string(), "8".to_string());
    input_to_char.insert("2s".to_string(), "9".to_string());
    input_to_char.insert("1s".to_string(), "0".to_string());

    // Bar and spacing
    input_to_char.insert("-".to_string(), "-".to_string());
    input_to_char.insert("6".to_string(), "6".to_string());

    //compas
    input_to_char.insert("4/4".to_string(), "ñ".to_string());
    input_to_char.insert("3/4".to_string(), "×".to_string());
    input_to_char.insert("2/4".to_string(), "ó".to_string());

    // LA
    input_to_char.insert("4A".to_string(), ")".to_string());
    input_to_char.insert("2A".to_string(), "c".to_string());
    input_to_char.insert("1A".to_string(), "o".to_string());

    input_to_char.insert("4Ab".to_string(), "?)".to_string());
    input_to_char.insert("2Ab".to_string(), "?c".to_string());
    input_to_char.insert("1Ab".to_string(), "?o".to_string());

    input_to_char.insert("4A#".to_string(), "Á)".to_string());
    input_to_char.insert("2A#".to_string(), "Ác".to_string());
    input_to_char.insert("1A#".to_string(), "Áo".to_string());

    // Si
    input_to_char.insert("4B".to_string(), "_".to_string());
    input_to_char.insert("2B".to_string(), "v".to_string());
    input_to_char.insert("1B".to_string(), "]".to_string());

    input_to_char.insert("4Bb".to_string(), "Ÿ_".to_string());
    input_to_char.insert("2Bb".to_string(), "Ÿv".to_string());
    input_to_char.insert("1Bb".to_string(), "Ÿ]".to_string());

    input_to_char.insert("4B#".to_string(), "Ë_".to_string());
    input_to_char.insert("2B#".to_string(), "Ëv".to_string());
    input_to_char.insert("1B#".to_string(), "Ëp".to_string());

    // Do
    input_to_char.insert("4C".to_string(), "+".to_string());
    input_to_char.insert("2C".to_string(), "l".to_string());
    input_to_char.insert("1C".to_string(), "r".to_string());

    input_to_char.insert("4Cb".to_string(), "·+".to_string());
    input_to_char.insert("2Cb".to_string(), "·l".to_string());
    input_to_char.insert("1Cb".to_string(), "·r".to_string());

    input_to_char.insert("4C#".to_string(), "È+".to_string());
    input_to_char.insert("2C#".to_string(), "Èl".to_string());
    input_to_char.insert("1C#".to_string(), "Èr".to_string());

    // Re

    input_to_char.insert("4D".to_string(), "^".to_string());
    input_to_char.insert("2D".to_string(), ";".to_string());
    input_to_char.insert("1D".to_string(), "t".to_string());

    input_to_char.insert("4Db".to_string(), "N^".to_string());
    input_to_char.insert("2Db".to_string(), "N;".to_string());
    input_to_char.insert("1Db".to_string(), "Nt".to_string());

    input_to_char.insert("4D#".to_string(), "„^".to_string());
    input_to_char.insert("2D#".to_string(), "„;".to_string());
    input_to_char.insert("1D#".to_string(), "„t".to_string());

    // Mi
    input_to_char.insert("4E".to_string(), "&".to_string());
    input_to_char.insert("2E".to_string(), "'".to_string());
    input_to_char.insert("1E".to_string(), "y".to_string());

    input_to_char.insert("4Eb".to_string(), "M&".to_string());
    input_to_char.insert("2Eb".to_string(), "M'".to_string());
    input_to_char.insert("1Eb".to_string(), "My".to_string());

    input_to_char.insert("4E#".to_string(), "‰&".to_string());
    input_to_char.insert("2E#".to_string(), "‰'".to_string());
    input_to_char.insert("1E#".to_string(), "‰y".to_string());

    // Fa
    input_to_char.insert("4F".to_string(), "*".to_string());
    input_to_char.insert("2F".to_string(), "z".to_string());
    input_to_char.insert("1F".to_string(), "u".to_string());

    input_to_char.insert("4Fb".to_string(), "<*".to_string());
    input_to_char.insert("2Fb".to_string(), "<z".to_string());
    input_to_char.insert("1Fb".to_string(), "<u".to_string());

    input_to_char.insert("4F#".to_string(), "Â*".to_string());
    input_to_char.insert("2F#".to_string(), "Âz".to_string());
    input_to_char.insert("1F#".to_string(), "Âu".to_string());

    // Sol
    input_to_char.insert("4G".to_string(), "(".to_string());
    input_to_char.insert("2G".to_string(), "x".to_string());
    input_to_char.insert("1G".to_string(), "i".to_string());

    input_to_char.insert("4Gb".to_string(), ">(".to_string());
    input_to_char.insert("2Gb".to_string(), ">x".to_string());
    input_to_char.insert("1Gb".to_string(), ">i".to_string());

    input_to_char.insert("4G#".to_string(), "Ê(".to_string());
    input_to_char.insert("2G#".to_string(), "Êx".to_string());
    input_to_char.insert("1G#".to_string(), "Êi".to_string());
    input_to_char
}
pub fn fa() -> HashMap<String, String> {
    let mut input_to_char = HashMap::new();

    // Claves
    input_to_char.insert("G".to_string(), "`".to_string());
    input_to_char.insert("F".to_string(), "1".to_string());
    input_to_char.insert("C1".to_string(), "2".to_string());
    input_to_char.insert("C2".to_string(), "3".to_string());

    //Silents
    input_to_char.insert("8s".to_string(), "7".to_string());
    input_to_char.insert("4s".to_string(), "8".to_string());
    input_to_char.insert("2s".to_string(), "9".to_string());
    input_to_char.insert("1s".to_string(), "0".to_string());

    // Bar and spacing
    input_to_char.insert("-".to_string(), "-".to_string());
    input_to_char.insert("6".to_string(), "6".to_string());

    //compas
    input_to_char.insert("4/4".to_string(), "ñ".to_string());
    input_to_char.insert("3/4".to_string(), "×".to_string());
    input_to_char.insert("2/4".to_string(), "ó".to_string());

    // LA
    input_to_char.insert("4C".to_string(), ")".to_string());
    input_to_char.insert("2C".to_string(), "c".to_string());
    input_to_char.insert("1C".to_string(), "o".to_string());

    input_to_char.insert("4Cb".to_string(), "?)".to_string());
    input_to_char.insert("2Cb".to_string(), "?c".to_string());
    input_to_char.insert("1Cb".to_string(), "?o".to_string());

    input_to_char.insert("4C#".to_string(), "Á)".to_string());
    input_to_char.insert("2C#".to_string(), "Ác".to_string());
    input_to_char.insert("1C#".to_string(), "Áo".to_string());

    // Si
    input_to_char.insert("4D".to_string(), "_".to_string());
    input_to_char.insert("2D".to_string(), "v".to_string());
    input_to_char.insert("1D".to_string(), "]".to_string());

    input_to_char.insert("4Db".to_string(), "Ÿ_".to_string());
    input_to_char.insert("2Db".to_string(), "Ÿv".to_string());
    input_to_char.insert("1Db".to_string(), "Ÿ]".to_string());

    input_to_char.insert("4D#".to_string(), "Ë_".to_string());
    input_to_char.insert("2D#".to_string(), "Ëv".to_string());
    input_to_char.insert("1D#".to_string(), "Ëp".to_string());

    // Do
    input_to_char.insert("4E".to_string(), "+".to_string());
    input_to_char.insert("2E".to_string(), "l".to_string());
    input_to_char.insert("1E".to_string(), "r".to_string());

    input_to_char.insert("4Eb".to_string(), "·+".to_string());
    input_to_char.insert("2Eb".to_string(), "·l".to_string());
    input_to_char.insert("1Eb".to_string(), "·r".to_string());

    input_to_char.insert("4E#".to_string(), "È+".to_string());
    input_to_char.insert("2E#".to_string(), "Èl".to_string());
    input_to_char.insert("1E#".to_string(), "Èr".to_string());

    // Re

    input_to_char.insert("4F".to_string(), "Q".to_string());
    input_to_char.insert("2F".to_string(), "n".to_string());
    input_to_char.insert("1F".to_string(), "]".to_string());

    input_to_char.insert("4Fb".to_string(), "N^".to_string());
    input_to_char.insert("2Fb".to_string(), "N;".to_string());
    input_to_char.insert("1Fb".to_string(), "Nt".to_string());

    input_to_char.insert("4F#".to_string(), "„^".to_string());
    input_to_char.insert("2F#".to_string(), "„;".to_string());
    input_to_char.insert("1F#".to_string(), "„t".to_string());

    // Mi
    input_to_char.insert("4G".to_string(), "&".to_string());
    input_to_char.insert("2G".to_string(), "'".to_string());
    input_to_char.insert("1G".to_string(), "y".to_string());

    input_to_char.insert("4Gb".to_string(), "M&".to_string());
    input_to_char.insert("2Gb".to_string(), "M'".to_string());
    input_to_char.insert("1Gb".to_string(), "My".to_string());

    input_to_char.insert("4G#".to_string(), "‰&".to_string());
    input_to_char.insert("2G#".to_string(), "‰'".to_string());
    input_to_char.insert("1G#".to_string(), "‰y".to_string());

    // Fa
    input_to_char.insert("4A".to_string(), "*".to_string());
    input_to_char.insert("2A".to_string(), "z".to_string());
    input_to_char.insert("1A".to_string(), "u".to_string());

    input_to_char.insert("4Ab".to_string(), "<*".to_string());
    input_to_char.insert("2Ab".to_string(), "<z".to_string());
    input_to_char.insert("1Ab".to_string(), "<u".to_string());

    input_to_char.insert("4A#".to_string(), "Â*".to_string());
    input_to_char.insert("2A#".to_string(), "Âz".to_string());
    input_to_char.insert("1A#".to_string(), "Âu".to_string());

    // Sol
    input_to_char.insert("4B".to_string(), "(".to_string());
    input_to_char.insert("2B".to_string(), "x".to_string());
    input_to_char.insert("1B".to_string(), "i".to_string());

    input_to_char.insert("4Bb".to_string(), ">(".to_string());
    input_to_char.insert("2Bb".to_string(), ">x".to_string());
    input_to_char.insert("1Bb".to_string(), ">i".to_string());

    input_to_char.insert("4B#".to_string(), "Ê(".to_string());
    input_to_char.insert("2B#".to_string(), "Êx".to_string());
    input_to_char.insert("1B#".to_string(), "Êi".to_string());
    input_to_char
}
pub fn do1() -> HashMap<String, String> {
    let mut input_to_char = HashMap::new();

    // Claves
    input_to_char.insert("G".to_string(), "`".to_string());
    input_to_char.insert("F".to_string(), "1".to_string());
    input_to_char.insert("C1".to_string(), "2".to_string());
    input_to_char.insert("C2".to_string(), "3".to_string());

    //Silents
    // input_to_char.insert("8s".to_string(), "7".to_string());
    input_to_char.insert("4s".to_string(), "8".to_string());
    input_to_char.insert("2s".to_string(), "9".to_string());
    input_to_char.insert("1s".to_string(), "0".to_string());

    // Bar and spacing
    input_to_char.insert("-".to_string(), "-".to_string());
    input_to_char.insert("6".to_string(), "6".to_string());

    //compas
    input_to_char.insert("4/4".to_string(), "ñ".to_string());
    input_to_char.insert("3/4".to_string(), "×".to_string());
    input_to_char.insert("2/4".to_string(), "ó".to_string());

    // LA
    input_to_char.insert("4B".to_string(), ")".to_string());
    input_to_char.insert("2B".to_string(), "c".to_string());
    input_to_char.insert("1B".to_string(), "o".to_string());

    input_to_char.insert("4Bb".to_string(), "?)".to_string());
    input_to_char.insert("2Bb".to_string(), "?c".to_string());
    input_to_char.insert("1Bb".to_string(), "?o".to_string());

    input_to_char.insert("4B#".to_string(), "Á)".to_string());
    input_to_char.insert("2B#".to_string(), "Ác".to_string());
    input_to_char.insert("1B#".to_string(), "Áo".to_string());

    // Si
    input_to_char.insert("4C".to_string(), "_".to_string());
    input_to_char.insert("2C".to_string(), "v".to_string());
    input_to_char.insert("1C".to_string(), "]".to_string());

    input_to_char.insert("4Cb".to_string(), "Ÿ_".to_string());
    input_to_char.insert("2Cb".to_string(), "Ÿv".to_string());
    input_to_char.insert("1Cb".to_string(), "Ÿ]".to_string());

    input_to_char.insert("4C#".to_string(), "Ë_".to_string());
    input_to_char.insert("2C#".to_string(), "Ëv".to_string());
    input_to_char.insert("1C#".to_string(), "Ëp".to_string());

    // Do
    input_to_char.insert("4D".to_string(), "+".to_string());
    input_to_char.insert("2D".to_string(), "l".to_string());
    input_to_char.insert("1D".to_string(), "r".to_string());

    input_to_char.insert("4Db".to_string(), "·+".to_string());
    input_to_char.insert("2Db".to_string(), "·l".to_string());
    input_to_char.insert("1Db".to_string(), "·r".to_string());

    input_to_char.insert("4D#".to_string(), "È+".to_string());
    input_to_char.insert("2D#".to_string(), "Èl".to_string());
    input_to_char.insert("1D#".to_string(), "Èr".to_string());

    // Re

    input_to_char.insert("4E".to_string(), "^".to_string());
    input_to_char.insert("2E".to_string(), ";".to_string());
    input_to_char.insert("1E".to_string(), "t".to_string());

    input_to_char.insert("4Eb".to_string(), "N^".to_string());
    input_to_char.insert("2Eb".to_string(), "N;".to_string());
    input_to_char.insert("1Eb".to_string(), "Nt".to_string());

    input_to_char.insert("4E#".to_string(), "„^".to_string());
    input_to_char.insert("2E#".to_string(), "„;".to_string());
    input_to_char.insert("1E#".to_string(), "„t".to_string());

    // Mi
    input_to_char.insert("4F".to_string(), "&".to_string());
    input_to_char.insert("2F".to_string(), "'".to_string());
    input_to_char.insert("1F".to_string(), "y".to_string());

    input_to_char.insert("4Fb".to_string(), "M&".to_string());
    input_to_char.insert("2Fb".to_string(), "M'".to_string());
    input_to_char.insert("1Fb".to_string(), "My".to_string());

    input_to_char.insert("4F#".to_string(), "‰&".to_string());
    input_to_char.insert("2F#".to_string(), "‰'".to_string());
    input_to_char.insert("1F#".to_string(), "‰y".to_string());

    // Fa
    input_to_char.insert("4G".to_string(), "*".to_string());
    input_to_char.insert("2G".to_string(), "z".to_string());
    input_to_char.insert("1G".to_string(), "u".to_string());

    input_to_char.insert("4Gb".to_string(), "<*".to_string());
    input_to_char.insert("2Gb".to_string(), "<z".to_string());
    input_to_char.insert("1Gb".to_string(), "<u".to_string());

    input_to_char.insert("4G#".to_string(), "Â*".to_string());
    input_to_char.insert("2G#".to_string(), "Âz".to_string());
    input_to_char.insert("1G#".to_string(), "Âu".to_string());

    // Sol
    input_to_char.insert("4A".to_string(), "(".to_string());
    input_to_char.insert("2A".to_string(), "x".to_string());
    input_to_char.insert("1A".to_string(), "i".to_string());

    input_to_char.insert("4Ab".to_string(), ">(".to_string());
    input_to_char.insert("2Ab".to_string(), ">x".to_string());
    input_to_char.insert("1Ab".to_string(), ">i".to_string());

    input_to_char.insert("4G#".to_string(), "Ê(".to_string());
    input_to_char.insert("2G#".to_string(), "Êx".to_string());
    input_to_char.insert("1G#".to_string(), "Êi".to_string());
    input_to_char
}
pub fn do2() -> HashMap<String, String> {
    let mut input_to_char = HashMap::new();

    // Claves
    input_to_char.insert("G".to_string(), "`".to_string());
    input_to_char.insert("F".to_string(), "1".to_string());
    input_to_char.insert("C1".to_string(), "2".to_string());
    input_to_char.insert("C2".to_string(), "3".to_string());

    //Silents
    input_to_char.insert("4s".to_string(), "8".to_string());
    input_to_char.insert("2s".to_string(), "9".to_string());
    input_to_char.insert("1s".to_string(), "0".to_string());

    // Bar and spacing
    input_to_char.insert("-".to_string(), "-".to_string());
    input_to_char.insert("6".to_string(), "6".to_string());

    //compas
    input_to_char.insert("4/4".to_string(), "ñ".to_string());
    input_to_char.insert("3/4".to_string(), "×".to_string());
    input_to_char.insert("2/4".to_string(), "ó".to_string());

    // LA
    input_to_char.insert("4G".to_string(), ")".to_string());
    input_to_char.insert("2G".to_string(), "c".to_string());
    input_to_char.insert("1G".to_string(), "o".to_string());

    input_to_char.insert("4Gb".to_string(), "?)".to_string());
    input_to_char.insert("2Gb".to_string(), "?c".to_string());
    input_to_char.insert("1Gb".to_string(), "?o".to_string());

    input_to_char.insert("4G#".to_string(), "Á)".to_string());
    input_to_char.insert("2G#".to_string(), "Ác".to_string());
    input_to_char.insert("1G#".to_string(), "Áo".to_string());

    // Si
    input_to_char.insert("4A".to_string(), "_".to_string());
    input_to_char.insert("2A".to_string(), "v".to_string());
    input_to_char.insert("1A".to_string(), "]".to_string());

    input_to_char.insert("4Ab".to_string(), "Ÿ_".to_string());
    input_to_char.insert("2Ab".to_string(), "Ÿv".to_string());
    input_to_char.insert("1Ab".to_string(), "Ÿ]".to_string());

    input_to_char.insert("4A#".to_string(), "Ë_".to_string());
    input_to_char.insert("2A#".to_string(), "Ëv".to_string());
    input_to_char.insert("1A#".to_string(), "Ëp".to_string());

    // Do
    input_to_char.insert("4B".to_string(), "+".to_string());
    input_to_char.insert("2B".to_string(), "l".to_string());
    input_to_char.insert("1B".to_string(), "r".to_string());

    input_to_char.insert("4Bb".to_string(), "·+".to_string());
    input_to_char.insert("2Bb".to_string(), "·l".to_string());
    input_to_char.insert("1Bb".to_string(), "·r".to_string());

    input_to_char.insert("4B#".to_string(), "È+".to_string());
    input_to_char.insert("2B#".to_string(), "Èl".to_string());
    input_to_char.insert("1B#".to_string(), "Èr".to_string());

    // Re

    input_to_char.insert("4C".to_string(), "^".to_string());
    input_to_char.insert("2C".to_string(), ";".to_string());
    input_to_char.insert("1C".to_string(), "t".to_string());

    input_to_char.insert("4Cb".to_string(), "N^".to_string());
    input_to_char.insert("2Cb".to_string(), "N;".to_string());
    input_to_char.insert("1Cb".to_string(), "Nt".to_string());

    input_to_char.insert("4C#".to_string(), "„^".to_string());
    input_to_char.insert("2C#".to_string(), "„;".to_string());
    input_to_char.insert("1C#".to_string(), "„t".to_string());

    // Mi
    input_to_char.insert("4D".to_string(), "&".to_string());
    input_to_char.insert("2D".to_string(), "'".to_string());
    input_to_char.insert("1D".to_string(), "y".to_string());

    input_to_char.insert("4Db".to_string(), "M&".to_string());
    input_to_char.insert("2Db".to_string(), "M'".to_string());
    input_to_char.insert("1Db".to_string(), "My".to_string());

    input_to_char.insert("4D#".to_string(), "‰&".to_string());
    input_to_char.insert("2D#".to_string(), "‰'".to_string());
    input_to_char.insert("1D#".to_string(), "‰y".to_string());

    // Fa
    input_to_char.insert("4E".to_string(), "*".to_string());
    input_to_char.insert("2E".to_string(), "z".to_string());
    input_to_char.insert("1E".to_string(), "u".to_string());

    input_to_char.insert("4Eb".to_string(), "<*".to_string());
    input_to_char.insert("2Eb".to_string(), "<z".to_string());
    input_to_char.insert("1Eb".to_string(), "<u".to_string());

    input_to_char.insert("4E#".to_string(), "Â*".to_string());
    input_to_char.insert("2E#".to_string(), "Âz".to_string());
    input_to_char.insert("1E#".to_string(), "Âu".to_string());

    // Sol
    input_to_char.insert("4F".to_string(), "(".to_string());
    input_to_char.insert("2F".to_string(), "x".to_string());
    input_to_char.insert("1F".to_string(), "i".to_string());

    input_to_char.insert("4Fb".to_string(), ">(".to_string());
    input_to_char.insert("2Fb".to_string(), ">x".to_string());
    input_to_char.insert("1Fb".to_string(), ">i".to_string());

    input_to_char.insert("4F#".to_string(), "Ê(".to_string());
    input_to_char.insert("2F#".to_string(), "Êx".to_string());
    input_to_char.insert("1F#".to_string(), "Êi".to_string());
    input_to_char
}
