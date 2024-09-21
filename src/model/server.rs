use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tsqp {
    pub tsqp: Server,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Server {
    #[serde(rename(deserialize = "@version", serialize = "version"))]
    version: String,
    motd: String,
    map: Map,
    owner: Owner,
    pub players: Players,
    rates: Option<Rates>,
    #[serde(rename = "serverinfo")]
    info: ServerInfo,
    monsters: Monsters,
    npcs: Option<Npcs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Map {
    #[serde(rename(deserialize = "@author", serialize = "author"))]
    author: String,
    #[serde(rename(deserialize = "@name", serialize = "name"))]
    name: String,
    #[serde(
        rename(deserialize = "@width", serialize = "width"),
        deserialize_with = "to_i64"
    )]
    width: i64,
    #[serde(
        rename(deserialize = "@height", serialize = "height"),
        deserialize_with = "to_i64"
    )]
    height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Monsters {
    #[serde(
        rename(deserialize = "@total", serialize = "total"),
        deserialize_with = "to_i64"
    )]
    total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Npcs {
    #[serde(
        rename(deserialize = "@total", serialize = "total"),
        deserialize_with = "to_i64"
    )]
    total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Owner {
    #[serde(rename(deserialize = "@email", serialize = "email"))]
    email: String,
    #[serde(rename(deserialize = "@name", serialize = "name"))]
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Players {
    #[serde(
        rename(deserialize = "@online", serialize = "online"),
        deserialize_with = "to_i64"
    )]
    pub online: i64,
    #[serde(
        rename(deserialize = "@max", serialize = "max"),
        deserialize_with = "to_i64"
    )]
    max: i64,
    #[serde(
        rename(deserialize = "@peak", serialize = "peak"),
        deserialize_with = "to_i64"
    )]
    peak: i64,
    #[serde(
        default,
        alias = "@unique",
        rename(deserialize = "@unique_players", serialize = "unique_players"),
        deserialize_with = "option_i64"
    )]
    pub unique_players: Option<i64>,
    #[serde(default)]
    pub bots: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    #[serde(rename(deserialize = "@client", serialize = "client"))]
    client: String,
    #[serde(rename(deserialize = "@ip", serialize = "ip"))]
    ip: String,
    #[serde(rename(deserialize = "@location", serialize = "location"))]
    location: String,
    #[serde(rename(deserialize = "@port", serialize = "port"))]
    port: String,
    #[serde(rename(deserialize = "@server", serialize = "server"))]
    server: String,
    #[serde(rename(deserialize = "@servername", serialize = "servername"))]
    servername: String,
    #[serde(
        rename(deserialize = "@uptime", serialize = "uptime"),
        deserialize_with = "to_i64"
    )]
    uptime: i64,
    #[serde(rename(deserialize = "@url", serialize = "url"))]
    url: String,
    #[serde(rename(deserialize = "@version", serialize = "version"))]
    version: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Rates {
    #[serde(
        rename(deserialize = "@experience", serialize = "experience"),
        deserialize_with = "to_f64"
    )]
    experience: f64,
    #[serde(
        rename(deserialize = "@loot", serialize = "loot"),
        deserialize_with = "to_f64"
    )]
    loot: f64,
    #[serde(
        rename(deserialize = "@magic", serialize = "magic"),
        deserialize_with = "to_f64"
    )]
    magic: f64,
    #[serde(
        rename(deserialize = "@skill", serialize = "skill"),
        deserialize_with = "to_f64"
    )]
    skill: f64,
    #[serde(
        rename(deserialize = "@spawn", serialize = "spawn"),
        deserialize_with = "to_f64"
    )]
    spawn: f64,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(0)
    }

    s.parse::<i64>().map_err(serde::de::Error::custom)
}

fn option_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    s.map(|s| {
        s.parse::<i64>()
            .map_err(|e| de::Error::custom(format!("Failed to parse i64: {}", e)))
    })
    .transpose()
}
