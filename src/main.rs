
use std::{error::Error, fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ServiceFabricClusters {
    #[serde(rename = "ServiceFabricCluster")]
    clusters: Vec<ServiceFabricCluster>,
}

#[derive(Debug, Deserialize)]
struct ServiceFabricCluster {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "GlobalValues")]
    global_values: GlobalValues,
}

#[derive(Debug, Deserialize)]
struct GlobalValues {
    #[serde(rename = "EnvironmentSetting")]
    env_settings: Vec<EnvironmentSetting>,
}

#[derive(Debug, Deserialize)]
struct EnvironmentSetting {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Value")]
    value: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(r"D:\OneDCMT\src\Configuration\NSM\NsmPlusSettings.Public.xml")?;
    let reader = BufReader::new(file);

    let clusters: ServiceFabricClusters = serde_xml_rs::from_reader(reader)?;

    for cluster in clusters.clusters {
        println!("{:?}", cluster);
    }

    Ok(())
}
