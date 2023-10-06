use anyhow::{Error, Result};
use clap::Parser;
use once_cell::sync::Lazy;
use reqwest::get;
use serde_json::Value;
use std::collections::HashMap;

static TEAM_LOOK_UP: Lazy<HashMap<String, u32>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ATL".to_string(), 1);
    m.insert("BUF".to_string(), 2);
    m.insert("CHI".to_string(), 3);
    m.insert("CIN".to_string(), 4);
    m.insert("CLE".to_string(), 5);
    m.insert("DAL".to_string(), 6);
    m.insert("DEN".to_string(), 7);
    m.insert("DET".to_string(), 8);
    m.insert("GB".to_string(), 9);
    m.insert("TEN".to_string(), 10);
    m.insert("IND".to_string(), 11);
    m.insert("KC".to_string(), 12);
    m.insert("LV".to_string(), 13);
    m.insert("LAR".to_string(), 14);
    m.insert("MIA".to_string(), 15);
    m.insert("MIN".to_string(), 16);
    m.insert("NE".to_string(), 17);
    m.insert("NO".to_string(), 18);
    m.insert("NYG".to_string(), 19);
    m.insert("NYJ".to_string(), 20);
    m.insert("PHI".to_string(), 21);
    m.insert("ARI".to_string(), 22);
    m.insert("PIT".to_string(), 23);
    m.insert("LAC".to_string(), 24);
    m.insert("SF".to_string(), 25);
    m.insert("SEA".to_string(), 26);
    m.insert("TB".to_string(), 28);
    m.insert("WSH".to_string(), 29);
    m.insert("CAR".to_string(), 30);
    m.insert("JAX".to_string(), 31);
    m
});

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    team: String,
    #[arg(short, long)]
    week: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
struct Team(String);

impl Team {
    fn from_str(name: &str) -> Self {
        Self(name.to_string().to_uppercase())
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct TeamMatchUp {
    matchup: String,
}

impl TeamMatchUp {
    fn new(matchup: String) -> Self {
        Self { matchup }
    }
    fn from_str(matchup: &str) -> Self {
        Self {
            matchup: matchup.to_string(),
        }
    }
}

impl std::fmt::Display for TeamMatchUp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matchup)
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
        Err(anyhow::anyhow!(
            "Could not determine the NFL team's schedule."
        ))
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, matchup) in self.matchups.iter().enumerate() {
            write!(f, "Week {}: {}\n", i + 1, matchup)?;
        }
        write!(f, "")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(team_code) = TEAM_LOOK_UP.get(&args.team.to_uppercase()) {
        let desired_team = Team::from_str(&args.team);
        let url = format!(
            "https://site.api.espn.com/apis/site/v2/sports/football/nfl/teams/{team_code}/schedule"
        );
        let espn_data = get(url).await?.text().await?;
        let schedule = Schedule::from_unstructed_json(&desired_team, &espn_data)?;
        println!("{schedule}");
        return Ok(());
    }
    Ok(())
}
