use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    version: String,
}

impl Version {
    pub fn version(&self) -> &String {
        &self.version
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Brands {
    #[serde(flatten)]
    inner: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageLookup {
    #[serde(flatten)]
    inner: HashMap<String, String>,
}

impl LanguageLookup {
    pub fn _get(&self, value: &str) -> &String {
        &self.inner[value]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainWeapon {
    addition: u32,
    id: u32,
    ink_saver_lv: String,
    lock: String,
    main_up_gear_power_type: String,
    move_vel_lv: String,
    name: String,
    param0: String,
    param1: String,
    param2: String,
    param_value0: u32,
    param_value1: u32,
    param_value2: u32,
    price: u32,
    range: u32,
    rank: u32,
    shot_move_vel_type: String,
    special: String,
    special_cost: u32,
    stealth_move_acc_lv: String,
    sub: String,
}

impl MainWeapon {
    pub fn _name(&self) -> &String {
        &self.name
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Special {
    pub id: u32,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sub {
    pub id: u32,
    pub ink_saver_type: String,
    pub name: String,
}

pub fn read_language<P: AsRef<Path>>(path: P) -> Result<LanguageLookup> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}

pub fn read_brands<P: AsRef<Path>>(path: P) -> Result<Brands> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}

pub fn read_main_weapons<P: AsRef<Path>>(path: P) -> Result<Vec<MainWeapon>> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}

pub fn read_specials<P: AsRef<Path>>(path: P) -> Result<Vec<Special>> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}

pub fn read_subs<P: AsRef<Path>>(path: P) -> Result<Vec<Sub>> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}

pub fn read_version<P: AsRef<Path>>(path: P) -> Result<Version> {
    let handle = File::open(path.as_ref())
        .with_context(|| format!("{:?} does not exist.", path.as_ref().to_str()))?;
    let reader = BufReader::new(handle);
    serde_json::from_reader(reader)
        .with_context(|| format!("{:?} data is malformed.", path.as_ref().to_str()))
}
