use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(name = "cao", about = "IP Update")]
pub struct Args {
    /// DNS API Provider.
    /// Only DNSPOD now.
    #[structopt(short, long)]
    pub provider: String,
    /// Token in file.
    /// The file only contains the token.
    #[structopt(short, long)]
    pub key: String,
    /// Domain
    #[structopt(short, long)]
    pub domain: String,
    /// sub command
    #[structopt(subcommand)]
    pub cmd: Cmds,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum Cmds {
    #[structopt(about = "Add a record")]
    Add {
        /// Subdomain
        #[structopt(short, long = "sub")]
        sub_domain: Option<String>,
        /// Record type
        #[structopt(short = "t", long = "type")]
        record_type: String,
        /// Record line
        #[structopt(short = "l", long = "line")]
        record_line: String,
        /// Value
        #[structopt(short, long)]
        value: Option<String>,
        /// Get value from interface
        #[structopt(long = "if")]
        interface: Option<String>,
    },
    #[structopt(about = "List records")]
    List {
        /// Offset
        #[structopt(short, long)]
        offset: Option<i32>,
        /// Length
        #[structopt(short, long)]
        length: Option<i32>,
    },
    #[structopt(about = "Modify a record")]
    Modify {
        /// Record ID
        #[structopt(short = "i", long = "id")]
        record_id: String,
        /// Subdomain
        #[structopt(short, long = "sub")]
        sub_domain: Option<String>,
        /// Record type
        #[structopt(short = "t", long = "type")]
        record_type: String,
        /// Record line
        #[structopt(short = "l", long = "line")]
        record_line: String,
        /// Value
        #[structopt(short, long)]
        value: Option<String>,
        /// Get value from interface
        #[structopt(long = "if")]
        interface: Option<String>,
    },
    #[structopt(about = "Delete a record")]
    Delete {
        /// Record ID
        #[structopt(short = "i", long = "id")]
        record_id: String,
    }
}

fn missing_if_or_value() -> clap::Error {
    clap::Error{
        message: String::from("error: Missing one of following required arguments:\n --if or --value"),
        kind: clap::ErrorKind::MissingRequiredArgument,
        info: None,
    }
}

impl Args {
    pub fn args() -> Result<Self, clap::Error> {

        let args = Self::from_args_safe()?;

        match &args.cmd {
            Cmds::Add{value, interface, ..} => {
                if (value.is_none() && interface.is_none()) {
                    return Err(missing_if_or_value());
                }
            },
            Cmds::Modify{value, interface, ..} => {
                if (value.is_none() && interface.is_none()) {
                    return Err(missing_if_or_value());
                }
            },
            _ => {}
        }

        Ok(args)
    }
}