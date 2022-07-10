use clap::{App, Arg};

fn main() {
    let matches = App::new("shout")
        .version("0.1.0")
        .author("Kishore Newton <kishorenkedev@gmail.com>")
        .about("echo for rust")
        .arg(
            Arg::with_name("text")    
                .value_name("TEXT")
                .help("Input text to return back")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")    
                .short("n")
                .long("no-newline")
                .help("Do not print a newline at the end of the output")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
