use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        print_help();
        std::process::exit(1);
    }

    let sink = match get_running_sink() {
        Some(sink) => sink,
        None => {
            println!("No sink found");
            std::process::exit(1);
        }
    };

    let mut cmd = Command::new("pactl");

    match args[0].trim() {
        x if x == "up" || x == "down" => {
            cmd.args(["set-sink-volume", sink.as_str()])
                .arg(if x == "up" {"+10%"} else {"-10%"} )
                .output().expect("Unable to run pactl");

            notify_volume(&sink);
        },
        x if x == "mute" || x == "unmute" || x == "toggle"=> {
            cmd.args(["set-sink-mute", &sink]);
            cmd.arg(match x {
                "mute" => "true",
                "unmute" => "false",
                _ => "toggle"
            });
            cmd.output().expect("Unable to mute or unmute");

            notify(&format!("Audio: {}", if is_mute(&sink) { "muted" } else {"unmuted"}))
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
        _ => Some("@DEFAULT_SINK@".to_string())
    }
}

fn get_volume(sink: &String) -> Option<String> {
    let output = Command::new("pactl")
        .args(["get-sink-volume", sink])
        .output().expect("Unable to get sink volume");

    if !output.status.success() {
        ()
    }

    let output_string = String::from_utf8(output.stdout).unwrap();

    let mut lines = output_string.lines();

    return match lines.next() {
        Some(str) => {
            let v = str.split(' ').nth(4).unwrap();
            Some(String::from(v))
        },
        None => None
    }
}

fn is_mute(sink: &String) -> bool {
    let output = Command::new("pactl")
        .args(["get-sink-mute", sink])
        .output().expect("Unable to get sink volume");

    if !output.status.success() {
        ()
    }

    let output_string = String::from_utf8(output.stdout).unwrap();

    println!("get mute: {}", output_string.trim_end());
    println!("get mute: {}", output_string.trim_end());

    return output_string.trim_end().ends_with("yes")
}

fn notify_volume(sink: &String) {
    match get_volume(&sink) {
        Some(volume) => {
            notify(&format!("Volume: {}", volume));
        },
        None => println!("Unable to get volume")
    };
}

fn notify(msg: &String) {
    Command::new("notify-send").arg(msg).output().expect("Unable to notify-send");
}

fn print_help() {
    println!("Available Commands: toggle, mute, unmute, increase, decrease")
}
