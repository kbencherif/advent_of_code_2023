use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
pub enum MapError {
    MapErrorUnexistant,
    MapErrorDefault,
}

impl std::error::Error for MapError {}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapError::MapErrorUnexistant => write!(f, "Unexistant Map type"),
            MapError::MapErrorDefault => write!(f, "Map error Default"),
        }
    }
}

#[derive(Debug, Clone)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Map {
    map_type: MapType,
    sources: Vec<u64>,
    dests: Vec<u64>,
    ranges: Vec<u64>,
}

impl Map {
    fn new(map_type: MapType) -> Self {
        Self {
            map_type,
            sources: Vec::new(),
            dests: Vec::new(),
            ranges: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_lines("input.txt")?;
    let seeds = seeds_from_string_to_vec(lines.remove(0));
    let almanac: Vec<Map> = separate_at_empty_string(lines)
        .into_iter()
        .map(|e| process_map(e))
        .collect();
    let mut result = seeds
        .iter()
        .map(|seed| find_location(*seed, &almanac))
        .collect::<Vec<u64>>();
    println!("{:?}", result.into_iter().min());

    result = seeds
        .chunks(2)
        .map(|e| {
            println!("seed: {}, range: {}", e[0], e[1]);
            let mut result = find_location(e[0], &almanac);
            for seed in e[0] + 1..e[1] + e[0] {
                let tmp = find_location(seed, &almanac);
                if tmp < result {
                    result = tmp
                }
            }
            Some(result)
        })
        .flatten()
        .collect();
    println!("{:?}", result.into_iter().min());
    Ok(())
}

fn get_map_type(s: &str) -> Result<MapType, MapError> {
    match s {
        "seed-to-soil" => Ok(MapType::SeedToSoil),
        "soil-to-fertilizer" => Ok(MapType::SoilToFertilizer),
        "fertilizer-to-water" => Ok(MapType::FertilizerToWater),
        "water-to-light" => Ok(MapType::WaterToLight),
        "light-to-temperature" => Ok(MapType::LightToTemperature),
        "temperature-to-humidity" => Ok(MapType::TemperatureToHumidity),
        "humidity-to-location" => Ok(MapType::HumidityToLocation),
        _ => Err(MapError::MapErrorUnexistant),
    }
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn seeds_from_string_to_vec(s: String) -> Vec<u64> {
    s.split_whitespace()
        .map(|n| n.parse::<u64>())
        .flatten()
        .collect()
}

fn separate_at_empty_string(original: Vec<String>) -> Vec<Vec<String>> {
    let mut result_vector: Vec<Vec<String>> = Vec::new();
    let mut subvector: Vec<String> = Vec::new();

    for mut elem in original.into_iter() {
        elem = elem.replace(" map:", "");
        if elem.is_empty() {
            if !subvector.is_empty() {
                result_vector.push(subvector.clone());
                subvector.clear();
            }
        } else {
            subvector.push(elem)
        }
    }
    result_vector.push(subvector);
    result_vector
}

fn process_map(mut infos: Vec<String>) -> Map {
    let e_map_type = get_map_type(infos.remove(0).as_str());

    if !e_map_type.is_ok() {
        panic!("Map type not ok {:?}", e_map_type);
    }
    let map_type = e_map_type.unwrap();
    let mut result = Map::new(map_type);
    infos.into_iter().for_each(|e| {
        let tmp = e
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        result.dests.push(tmp[0]);
        result.sources.push(tmp[1]);
        result.ranges.push(tmp[2]);
    });
    result
}

fn find_location(seed: u64, almanac: &Vec<Map>) -> u64 {
    let mut result = seed;

    for map in almanac {
        for i in 0..map.sources.len() {
            if result >= map.sources[i] && result <= map.sources[i] + map.ranges[i] {
                result = result - map.sources[i] + map.dests[i];
                break;
            }
        }
    }
    result
}
