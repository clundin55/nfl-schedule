use anyhow::{Error, Result};
use reqwest::get;
use serde_json::Value;

const TOTAL_NFL_TEAMS: u32 = 32;
const REGULAR_SEASON_WEEKS: u32 = 7;

#[derive(Debug, Clone)]
struct Team(String);

impl Team {
    fn from_unstructed_json(data: &str) -> Result<Self, Error> {
        let json: Value = serde_json::from_str(data)?;
        let abbrev = json
            .get("team")
            .and_then(|team| team.get("abbreviation"))
            .and_then(|abbrev| abbrev.as_str());

        if let Some(abbrev) = abbrev {
            return Ok(Self(abbrev.to_string()));
        }
        Err(anyhow::anyhow!("Could not determine the NFL team."))
    }
    fn from_str(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug)]
struct TeamMatchUp {
    matchup: String,
}

impl TeamMatchUp {
    fn new(matchup: String) -> Self {
        Self {matchup}
    }
    fn from_str(matchup: &str) -> Self {
        Self{matchup: matchup.to_string()}
    }
}

#[derive(Debug)]
struct Schedule {
    matchups: Vec<TeamMatchUp>,
}

impl Schedule {
    fn from_unstructed_json(home_team: &Team, data: &str) -> Result<Self, Error> {
        let json: Value = serde_json::from_str(data)?;
        let matchups = json
            .get("events")
            .and_then(|events| events.as_array())
            .and_then(|events| Some(events.iter().filter_map(|event| event.get("shortName"))));

        if let Some(matchups) = matchups {
            let matchups = matchups
                .filter_map(|matchup| {
                    if let Some(matchup) = matchup.as_str() {
                        return Some(TeamMatchUp::from_str(matchup));
                    }
                    None
                })
                .collect();
            return Ok(Self { matchups });
        }
        Err(anyhow::anyhow!("Could not determine the NFL team's schedule."))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let espn_data =
        get("https://site.api.espn.com/apis/site/v2/sports/football/nfl/teams/4/schedule")
            .await?
            .text()
            .await?;
    let team = Team::from_unstructed_json(&espn_data)?;
    let schedule = Schedule::from_unstructed_json(&team, &espn_data)?;

    //println!("{espn_data}");
    println!("{team:#?}");
    println!("{schedule:#?}");
    Ok(())
}
