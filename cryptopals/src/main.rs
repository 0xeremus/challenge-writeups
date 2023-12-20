use clap::{App, Arg};

// import our challenges
mod set1;
mod set2;
mod utils;

fn main() {
    let matches = App::new("Cryptopals Challenges")
        .version("1.0")
        .about("Eremus's results from working through the Matasano Cryptopal Challenges")
        .arg(
            Arg::with_name("challenge")
                .help("The number of the challenge to execute [default is all]")
                .required(true),
        )
        .arg(Arg::with_name("describe").help("Print the challenge question"))
        .get_matches();

    let challenge = matches.value_of("challenge").unwrap().trim(); // Clap will give something or help

    match matches.value_of("describe") {
        Some(_) => std::env::set_var("DESCRIBE", "true"),
        None => (),
    };

        match challenge {
            "1" => set1::ch1(),
            "2" => set1::ch2(),
            "3" => set1::ch3(),
            "4" => set1::ch4(),
            "5" => set1::ch5(),
            "6" => set1::ch6(),
            "7" => set1::ch7(),
            "8" => set1::ch8(),
            "9" => set2::ch9(),
            "10" => set2::ch10(),

            "all" => {
                set1::ch1();
                println!();
                set1::ch2();
                println!();
                set1::ch3();
                println!();
                set1::ch4();
                println!();
                set1::ch5();
                println!();
                set1::ch6();
                println!();
                set1::ch7();
                println!();
                set1::ch8();
                println!();
                set2::ch9();
                println!();
                set2::ch10();
            }
            _ => unimplemented!(),
    }
}
