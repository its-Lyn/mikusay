use std::{env, error::Error, fmt::Write};

fn make_speech_bubble(format_message: String, biggest_width: i32) -> Result<String, Box<dyn Error>> {
    let bubble_wrapper = format!(" {}", "-".repeat((biggest_width + 1) as usize));
    let mut speech_bubble: String = format!("{}\n", &bubble_wrapper);
    
    for (lines_parsed, line) in format_message.lines().enumerate() {
        let space = String::from(" ").repeat(biggest_width as usize - line.len());
        if lines_parsed == 0 {
            writeln!(&mut speech_bubble, "/ {line}{space}\\")?;
        } else if lines_parsed == format_message.lines().count() - 1 {
            writeln!(&mut speech_bubble, "\\ {line}{space}/")?;
        } else {
            writeln!(&mut speech_bubble, "| {line}{space}|")?;
        }
    }

    speech_bubble.push_str(&bubble_wrapper);
    Ok(speech_bubble)
}

fn process_message(message: &str) -> Result<(), Box<dyn Error>> {
    const MAX_WIDTH: i32 = 50;

    // Calculate widths
    let mut current_width: i32 = 0;
    let mut biggest_width: i32 = 0;
    let mut message_with_break: String = String::new();

    for token in message.split_whitespace() {
        current_width += (token.len() + 1) as i32;
        if current_width > biggest_width {
            biggest_width = current_width;
        }

        write!(&mut message_with_break, "{token} ")?;

        if current_width > MAX_WIDTH { 
            current_width = 0;
            message_with_break.push('\n');
        }
    }

    println!("{}", make_speech_bubble(message_with_break, biggest_width)?);

    Ok(())
}

fn main() {
    let miku_data: [&str; 17] = [
        " \\⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⠊⣠⠖⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠞⢁⡔⡳⢌⣢⡀⠀⠀⠀⠀⠀⠀⠀⠀",
        "  \\⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⣠⣞⠵⠂⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⣰⠃⢠⢎⠏⠀⠀⠙⡟⢆⠀⠀⠀⠀⠀⠀⠀",
        "   \\⠀⠀⠀⠀⠀⠀⠈⠉⡿⣿⠋⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⠀⢫⠑⡇⢸⠀⠀⠀⠀⠘⣼⠃⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⢱⠇⠀⠀⠀⢠⡇⡶⠀⠀⠀⠀⠀⢸⡀⠀⢰⠀⠀⠀⠀⢣⠘⣼⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠀⡜⠀⠀⠀⣠⡞⢫⣷⠀⠀⠀⠀⠀⡞⡏⠙⣿⡆⠀⠀⠀⠈⢆⢹⠀⠀⠀⠀⠀⠀⢱⡀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⢸⠁⢰⠁⠀⠀⠈ ⣿⣇⣸⣿⡆⠀⠀⢀⣘⣧⣱⠀⢸⢃⠀⠀⠀⠀⡌⢮⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⡇⠀⠀⠀⠀⣿⣿⣿⣷⠹⡄⠀⠈⢿⣿⣿⣷⣾⢼⠀⠀⠀⢠⠃⠈⡆⠀⠀⠀⠀⠀⠀⠸⡀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⡞⠀⢰⠁⢠⡇⠀⢸⣿⢿⣿⣿⠁⠑⢄⠀⡸⢹⣿⣿⣿⣿⠀⢰⠀⡞⢣⠀⡇⠀⠀⠀⠀⠀⠀⠀⣇⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⡼⠀⠀⠈⡄⡏⡇⠀⠈⣿⠬⠿⠏⢠⠀⠈⠳⠇⠸⠿⡿⠛⢻⢠⡟⡰⠁⢸⠀⢸⠀⠀⠀⠀⠀⠀⠀⠸⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⡼⠁⠀⠀⠀⢻⠀⢱⡀⡀⢸⡄⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀⣮⢎⡿⡁⠴⠋⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀",
        "⠀⠀⠀⠀⡼⠁⠀⠀⠀⠀⠀⠃⠀⡧⣸⢆⢳⣄⠀⠀⠻⡟⢙⡿⠂⠀⠀⠞⠁⢚⡽⠁⠀⣠⠔⠉⡄⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀",
        "⠀⠀⠀⠊⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⠉⠀⠙⠀⠑⢤⡀⠀⠀⠀⠀⢀⡠⠔⠚⣯⣴⠒⠈⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠈⡆⠀⠀",
        "⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠈⠑⠒⣞⠉⠀⠀⠀⠀⡷⢌⣧⠀⠀⠀⠀⣧⠀⠀⠀⠀⠀⠀⠀⠀⢇⠀⠀",
        "⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⢀⣠⠖⢉⣽⠀⠀⠀⠀⣀⡷⠆⠈⣆⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀",
        "⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⣠⢴⠶⠖⢪⠟⠁⠀⡞⠁⢀⠤⠒⠉⠀⠀⠀⠀⡼⢧⡀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⠀⢀⠞⠀⡸⠀⠀⡎⠀⣀⣸⡗⠊⠁⠀⠀⠀⠀⠀⢀⠞⠀⠀⠙⠶⣼⠀⠀⠀⠀⠀⠀⠀⠀⠀⢡⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡇⢀⠎⠀⣠⠃⠀⢰⠖⣻⠁⠀⢸⠳⣄⠀⠀⠀⢀⡴⠃⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡀",
    ];

    let mut message: &str = "I'm a woman! Please provide me a message.";
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        message = &args[1];
    }

    if let Err(e) = process_message(message) {
        eprintln!("Failed to run mikusay {e}");
    }
 
    for line in miku_data {
        println!("{line}");
    }
}
