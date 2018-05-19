use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("sbanken-cli")
        .version("0.2.0")
        .about("Your personal bank right in your favorite terminal")
        .author("John-Olav Storvold")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets level of verbosity"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Allows for colored output. Equal to SBANKEN_COLOR=1."),
        )
        .subcommand(
            SubCommand::with_name("account")
                .about("See account details")
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .long("account")
                        .required(false)
                        .help("Retrieve details for a specified account")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .required(false)
                        .help("Retrieve accounts as a list"),
                )
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("customer")
                .about("display customer information")
                .display_order(2),
        )
        .subcommand(
            SubCommand::with_name("transaction")
                .about("See transactions made on your accounts")
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .long("account")
                        .required(true)
                        .help("List transactions made on your account")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .long("from")
                        .help(
                            "An start date, yyyy-mm-dd, to be used to narrow the results.\n\
                             Defaults to current time and date minus 30 days.",
                        )
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .help(
                            "An end date, yyyy-mm-dd, to be used to narrow the results.\n\
                             Defaults to current time and date.",
                        )
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("length")
                        .help("Number of transactions to be displayed")
                        .default_value("20")
                        .takes_value(true),
                )
                .display_order(3),
        )
        .subcommand(
            SubCommand::with_name("transfer")
                .about("Transfer between your accounts")
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .long("from")
                        .required(true)
                        .takes_value(true)
                        .help("From account you want to withdraw money from"),
                )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .required(true)
                        .takes_value(true)
                        .help("To account you want to deposit money into"),
                )
                .arg(
                    Arg::with_name("amount")
                        .short("a")
                        .long("amount")
                        .required(true)
                        .takes_value(true)
                        .help("Amount to transfer between accounts"),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .required(true)
                        .takes_value(true)
                        .help("Message to be recorded"),
                )
                .display_order(4),
        )
        .subcommand(
            SubCommand::with_name("generate-bash-completions")
                .about("Generate completion script for bash")
                .display_order(5),
        )
        .subcommand(
            SubCommand::with_name("generate-zsh-completions")
                .about("Generate completion script for zsh")
                .display_order(6),
        )
        .subcommand(
            SubCommand::with_name("generate-fish-completions")
                .about("Generate completion script for fish")
                .display_order(7),
        )
        .subcommand(
            SubCommand::with_name("generate-powershell-completions")
                .about("Generate completion script for PowerShell")
                .display_order(8),
        )
}
