struct CommandDef {
    path: String,
    attrs: Vec<String>,
}

fn main() {
    let command_defs = vec![
        CommandDef {
            path: String::from("/usr/bin/ls"),
            attrs: vec![String::from("-l"), String::from("-a")],
        },
        CommandDef {
            path: String::from("/usr/bin/grep"),
            attrs: vec![String::from("-i"), String::from("--color")],
        },
    ];

    let (paths, attrs): (Vec<String>, Vec<Vec<String>>) = command_defs
        .into_iter()
        .map(|cmd| (cmd.path, cmd.attrs))
        .unzip();

    println!("Paths: {:?}", paths);
    println!("Attrs: {:?}", attrs);
}
