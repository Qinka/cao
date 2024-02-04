use structopt::StructOpt;
use structopt::clap;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(name = "cao", about = "IP Update")]
pub enum Args {
    #[structopt(about = "Record operation")]
    Record {
        /// DNS API Provider.
        /// Only DNSPOD now.
        #[structopt(short, long)]
        provider: String,
        /// Token in file.
        /// The file only contains the token.
        #[structopt(short, long)]
        key: String,
        /// Domain
        #[structopt(short, long)]
        domain: String,
        /// sub command
        #[structopt(subcommand)]
        cmd: RecordCmds,
    },
    #[structopt(about = "List interfaces")]
    Interface {
        #[structopt(short, long)]
        interface: Option<String>
    },
}



#[derive(Debug, StructOpt, PartialEq)]
pub enum RecordCmds {
    #[structopt(about = "Add a record")]
    Add {
        /// Subdomain
        #[structopt(short, long = "sub")]
        sub_domain: String,
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
        /// Subdomain
        #[structopt(short, long)]
        sub_domain: Option<String>,
    },
    #[structopt(about = "Modify a record")]
    Modify {
        /// Record ID
        #[structopt(short = "i", long = "id")]
        record_id: u64,
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
        record_id: u64,
    },
}

fn missing_if_or_value() -> clap::Error {
    clap::Error{
        message: String::from("error: Missing one of following required arguments:\n --if or --value"),
        kind: clap::ErrorKind::MissingRequiredArgument,
        info: None,
    }
}

impl Args {
    pub fn get_args() -> Result<Self, clap::Error> {

        let args = Self::from_args_safe()?;

        if let Args::Record{cmd, ..} = &args {
            match cmd {
                RecordCmds::Add{value, interface, ..} => {
                    if value.is_none() && interface.is_none() {
                        return Err(missing_if_or_value());
                    }
                },
                RecordCmds::Modify{value, interface, ..} => {
                    if value.is_none() && interface.is_none() {
                        return Err(missing_if_or_value());
                    }
                },
                _ => {}
            }
        }

        Ok(args)
    }
}