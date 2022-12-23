use std::str::FromStr;

use anyhow::{Error, Result, Context};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub valves: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (?P<valve>.*)? has flow rate=(?P<flow>-?\d+)?; tunnels? leads? to valves? (?P<valves>.*)?$").unwrap();
        }

        let (name, flow_rate, valves) = RE
            .captures(s)
            .and_then(|cap| {
                let name = cap.name("valve").map(|v| v.as_str())?.to_owned();
                let flow_rate = cap
                    .name("flow")
                    .map(|v| v.as_str().parse::<usize>())?
                    .ok()?;
                let valves = cap.name("valves").map(|v| v.as_str())?;

                Some((name, flow_rate, valves))
            })
            .context("Error during parse")?;

        let valves = if valves.contains(',') {
            valves.split(", ").map(|v| v.to_owned()).collect::<Vec<_>>()
        } else {
            vec![valves.to_owned()]
        };

        Ok(Valve {
            name,
            flow_rate,
            valves,
        })
    }
}

