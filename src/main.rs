use structopt::StructOpt;



#[derive(StructOpt, Debug)]
struct Cli{
    command: u32,
    data: String,
}

fn main() {
    let cli = Cli::from_args();
    match cli.command {
        1 => {
            println!("this is encrytion");
        }
        2 => {
            println!("this is decrytion");
        }
        _ => {
            println!("please input 1(encrytion)/2(decrytion)");
        }
    }
    println!("{:#?}",cli);
}
