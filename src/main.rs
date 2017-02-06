#[macro_use]
extern crate clap;
extern crate pagersduty;

use std::process;
use std::fmt;
use std::result;
use std::io::Write;

use clap::{Arg, App, SubCommand};


/// Custom Result used for pdctl
pub type Result<T> = result::Result<T, Error>;


/// Custom Error type tha
#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Message(ref msg) => write!(f, "{}", msg),
        }
    }
}


fn parse_args() -> clap::ArgMatches<'static> {
    App::new("pdctl")
        .version(crate_version!())
        .about("Command-line tool for interacting with the PagerDuty API")
        .arg(Arg::with_name("verbose")
           .short("v")
           .long("verbose")
           .multiple(true)
           .help("Sets the level of verbosity")
        )
        .subcommand(SubCommand::with_name("events")
            .about("Interact with the Events API")
            .arg(Arg::with_name("service_key")
                .long("service-key")
                .value_name("KEY")
                .help("The GUID of one of your 'Generic API' services.")
                .takes_value(true)
                .required(true)
            )
            .subcommand(SubCommand::with_name("trigger")
                .about("Trigger an Event")
                .arg(Arg::with_name("description")
                    .long("description")
                    .value_name("TEXT")
                    .help("Text that will appear in the log with the event.")
                    .takes_value(true)
                    .required(true)
                )
            )
            .subcommand(SubCommand::with_name("acknowledge")
                .about("Acknowledge an Event")
                .arg(Arg::with_name("incident_key")
                    .long("incident-key")
                    .value_name("KEY")
                    .help("Identifies the incident to acknowledge.")
                    .takes_value(true)
                    .required(true)
                )
            )
            .subcommand(SubCommand::with_name("resolve")
                .about("Resolve an Event")
                .arg(Arg::with_name("incident_key")
                    .long("incident-key")
                    .value_name("KEY")
                    .help("Identifies the incident to resolve.")
                    .takes_value(true)
                    .required(true)
                )
            )
        )
        .get_matches()
}

fn trigger_event(service_key: String, description: String) {
    let response = pagersduty::events::TriggerEvent::new(
        service_key, description
    ).send();
}

fn acknowledge_event(service_key: String, incident_key: String) {
    let response = pagersduty::events::AcknowledgeEvent::new(
        service_key, incident_key
    ).send();
}

fn resolve_event(service_key: String, incident_key: String) {
    let response = pagersduty::events::ResolveEvent::new(
        service_key, incident_key
    ).send();
}



fn run() -> Result<()> {
    let matches = parse_args();

    if let Some(matches) = matches.subcommand_matches("events") {
        let service_key = matches.value_of("service_key").unwrap().into();
        match matches.subcommand() {
            ("trigger", Some(matches)) => {
                let description = matches.value_of("description").unwrap().into();
                trigger_event(service_key, description);
            },
            ("acknowledge", Some(matches)) => {
                let incident_key = matches.value_of("incident_key").unwrap().into();
                acknowledge_event(service_key, incident_key);
            },
            ("resolve", Some(matches)) => {
                let incident_key = matches.value_of("incident_key").unwrap().into();
                resolve_event(service_key, incident_key);
            },
            _ => panic!(),
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            let _ = writeln!(&mut ::std::io::stderr(), "{}", err);
            process::exit(1);
        }
    }
}
