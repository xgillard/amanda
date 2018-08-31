extern crate clap;
extern crate amanda;

use clap::*;

fn main() {

    // STUDENT // This should be displayed instead of the actual code

    // BEGIN STRIP
    let args = App::new("amanda")
        .version("1.0")
        .about("This is amanda the stripper, it strips away bits of your code...")
        .author("Xavier Gillard")
        .arg(Arg::with_name("from")
            .short("f")
            .long("from")
            .value_name("ORGINIAL SOURCE DIRECTORY")
            .help("The path to the directory containing the original (unstripped) code.")
            .takes_value(true)
        )
        .arg(Arg::with_name("to")
            .short("t")
            .long("to")
            .value_name("TARGET DIRECTORY")
            .help("The path to the directory where to produce a stripped copy of the code")
            .takes_value(true)
        )
        .get_matches();



    amanda::strip_tree(args.value_of("from").unwrap(),
                       args.value_of("to").unwrap());
    // END STRIP
}
