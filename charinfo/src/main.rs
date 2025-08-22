#[derive(Debug)]
pub struct CharInfo {
    index: usize,
    character: char,
}

fn main() {
    let v = vec!['A', 'B', 'C', 'D', 'E'];
    let mut char_infos = Vec::new();
    let mut ix = 0;
    for c in v.iter() {
        char_infos.push(CharInfo {
            index: ix,
            character: *c,
        });
        ix += 1;
    }
    char_infos.iter().rev().for_each(|char_info| {
        println!("{:?}", char_info);
    });
    println!("Char Infos: {:?}", char_infos)
}
