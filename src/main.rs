fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        print_help();
        std::process::exit(1);
    }

    let sink = match get_running_sink() {
        Some(sink) => sink,
        None => std::process::exit(1)
    };

    let mut cmd = std::process::Command::new("pactl");

     match args[0].trim() {
        x if x == "up" || x == "down" => {
                cmd.args(["set-sink-volume", sink.as_str()])
                .arg(if x == "up" {"+10%"} else {"-10%"} )
                .output().expect("Unable to run pactl");
        },
        x if x == "mute" || x == "unmute" => {
            cmd.args(["set-sink-mute", if x == "mute" {"true"} else {"false"}]);
            cmd.output().expect("Unable to mute or unmute");
        },
        _ => println!("{}", args[0])
    };
}

fn get_running_sink() -> Option<String> {
    let mut cmd = std::process::Command::new("pactl");
    cmd.args(["list", "short", "sinks"]);

    let output = cmd.output().expect("Unable to get that");

    if !output.status.success() {
        return None
    }

    let output_string = String::from_utf8(output.stdout).unwrap();
    let mut running_sink = output_string.lines().filter(|l| l.ends_with("RUNNING"));

    match running_sink.next() {
        Some(str) => {
            let l = str.split('\t').nth(1).unwrap();
            Some(String::from(l))
        },
        _ => None
    }
}

fn print_help() {
    println!("Available Commands: toggle, mute, unmute, increase, decrease")
}
