use csv::{Reader, StringRecord};
use std::fs::File;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Quiz")]
struct Opt {
    /// File with questions
    #[structopt(short = "c", long, default_value = "./problems.csv")]
    csvfile: String,
}

fn main() -> std::io::Result<()> {
    let args = Opt::from_args();
    let mut correct_answers = 0;

    let file = File::open(&args.csvfile)?;
    let mut reader = Reader::from_reader(file);
    let mut input = String::new();
    let mut record = StringRecord::new();

    while reader.read_record(&mut record)? {
        input.clear();
        let question = &record[0];
        let answer = &record[1];

        println!("{}", question);
        io::stdin().read_line(&mut input)?;

        if input == answer {
            correct_answers += 1;
        }
    }

    println!("Total: {}", &correct_answers);

    // Read all, but we want to read one by one.
    // for result in reader.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    Ok(())
}
