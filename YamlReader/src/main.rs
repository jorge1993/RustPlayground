use serde::{Deserialize, Deserializer};
use std::time::Duration;
use std::{env, fs, io, io::Read};


fn get_default_event_timeout() -> f32 {
    0.0
}

fn get_default_timeout() -> f32 {
    5.0
}

fn get_default_beacon() -> f32 {
    2.0
}

const CFG_VERSION_MIN: u16 = 2;
const CFG_VERSION_MAX: u16 = 2;

#[derive(Deserialize)]
struct GenProto {
    name: String,
}

fn de_output<'de, D>(
    deserializer: D,
) -> serde::export::Result<(pl::datatypes::OutputType, pl::datatypes::OutputFlags), D::Error>
where
    D: Deserializer<'de>,
{
    return Ok(
        pl::datatypes::get_output_type(&String::deserialize(deserializer).unwrap()).unwrap(),
    );
}

fn de_time_format<'de, D>(
    deserializer: D,
) -> serde::export::Result<pl::datatypes::TimeFormat, D::Error>
where
    D: Deserializer<'de>,
{
    return Ok(
        pl::datatypes::get_time_format(&String::deserialize(deserializer).unwrap()).unwrap(),
    );
}


#[derive(Deserialize)]
struct Config {
    version: u16,
    subversion: u16,
    parch: u16,
    #[serde(default = "get_default_timeout")]
    timeout: f32,
    #[serde(alias = "event-timeout", default = "get_default_event_timeout")]
    event_timeout: f32,
    #[serde(default = "get_default_beacon")]
    beacon: f32,
    freq: f64,
    resend: Option<f32>,
    #[serde(
        default = "pl::datatypes::get_default_output",
        deserialize_with = "de_output"
    )]
    output: (pl::datatypes::OutputType, pl::datatypes::OutputFlags),
    proto: GenProto,
    #[serde(
        alias = "time-format",
        default = "pl::datatypes::get_default_time_format",
        deserialize_with = "de_time_format"
    )]
    time_format: pl::datatypes::TimeFormat,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Cambia el nombre del archivo YAML que deseas leer.
    let file_path = "data/config.yaml";

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parsear el archivo YAML en una instancia de la estructura definida.
    let config: Config = serde_yaml::from_str(&contents)?;

    // Ahora puedes acceder a los datos del archivo YAML en `config`.
    // Por ejemplo:
    println!("Campo 1: {}", config.campo1);
    println!("Campo 2: {}", config.campo2);

    Ok(())
}