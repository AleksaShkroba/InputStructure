use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};
use std::fmt::Error;

use std::fs::File;
use std::io::{BufReader, Read, Write};


fn readLine<T: FromStr>() -> T
    where
        T::Err: Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse::<T>() {
        Ok(value) => value,
        Err(err) => panic!(
            "Failed to parse {} as {}. {:?}",
            line,
            std::any::type_name::<T>(),
            err
        ),
    }
}


#[derive(Deserialize, Serialize, Debug)]
struct Track {
    name: String,
    author: String,
    plays: u64,
    likes: u64,
}

fn readFile(inputNameFile: String) -> String {
    let mut file = File::open(inputNameFile).expect("Can't open file");

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Can't read file");

    return data;
}

fn fromJson(inputNameFile: String) -> Track {
    let mut data = readFile(inputNameFile);
    let track: Track = serde_json::from_str(&data).expect("Json was not well formatted");

    return track;
}

fn fromRon(inputNameFile: String) -> Track {
    let mut data = readFile(inputNameFile);
    let track: Track = ron::from_str(&data).unwrap();

    return track;
}

fn fromYaml(inputNameFile: String) -> Track {
    let mut data = readFile(inputNameFile);
    let track: Track = serde_yaml::from_str(&data).unwrap();

    return track;
}

fn fromToml(inputNameFile: String) -> Track {
    let mut data = readFile(inputNameFile);
    let track: Track = toml::from_str(&data).unwrap();

    return track;
}

fn toStruct(inputNameFile: String, inputFormat: String) -> Track {
    let mut track: Track;

    if inputFormat == "json" {
        track = fromJson(inputNameFile);
    } else if inputFormat == "ron" {
        track = fromRon(inputNameFile);
    } else if inputFormat == "yaml" {
        track = fromYaml(inputNameFile);
    } else if inputFormat == "toml" {
        track = fromToml(inputNameFile);
    } else {
        panic!("noInputFileFormat");
    }
    return track;
}


fn toJson(outputNameFile: String, structure: Track) {
    let mut file = File::create(outputNameFile).expect("Can't open file");

    let data = serde_json::to_string(&structure).unwrap();
    file.write_all(data.as_bytes()).expect("Can't write data to file");
}

fn toRon(outputNameFile: String, structure: Track) {
    let mut file = File::create(outputNameFile).expect("Can't open file");

    let data = ron::to_string(&structure).unwrap();
    file.write_all(data.as_bytes()).expect("Can't write data to file");
}

fn toYaml(outputNameFile: String, structure: Track) {
    let mut file = File::create(outputNameFile).expect("Can't open file");

    let data = serde_yaml::to_string(&structure).unwrap();
    file.write_all(data.as_bytes()).expect("Can't write data to file");
}

fn toToml(outputNameFile: String, structure: Track) {
    let mut file = File::create(outputNameFile).expect("Can't open file");

    let data = toml::to_string(&structure).unwrap();
    file.write_all(data.as_bytes()).expect("Can't open file");
}

fn toFile(outputNameFile: String, outputFormat: String, structure: Track) {

    if outputFormat == "json" {
        toJson(outputNameFile, structure);
    } else if outputFormat == "ron" {
        toRon(outputNameFile, structure);
    } else if outputFormat == "yaml" {
        toYaml(outputNameFile, structure);
    } else if outputFormat == "toml" {
        toToml(outputNameFile, structure);
    }
}

fn makeFile(inputNameFile: String, inputFormat: String,
            outputNameFile: String, outputFormat: String) {

    let mut track = toStruct(inputNameFile, inputFormat);

    toFile(outputNameFile, outputFormat, track);
}

fn main() {
    let inputNameFile: String = readLine();
    let inputFormat: String  = readLine();
    let outputNameFile: String = readLine();
    let outputFormat: String  = readLine();

    makeFile(inputNameFile, inputFormat, outputNameFile, outputFormat);
}