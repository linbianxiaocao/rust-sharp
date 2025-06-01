
use std::{error::Error, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServiceFabricClusters {
    #[serde(rename = "ServiceFabricCluster")]
    clusters: Vec<ServiceFabricCluster>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceFabricCluster {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "GlobalValues")]
    global_values: GlobalValues,
}

#[derive(Debug, Serialize, Deserialize)]
struct GlobalValues {
    #[serde(rename = "EnvironmentSetting")]
    env_settings: Vec<EnvironmentSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
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

    for cluster in &clusters.clusters {
        for env_setting in &cluster.global_values.env_settings {
            if env_setting.name == "DstsApplicationPrincipalId" {
                println!("{}, {}: {}", cluster.name, env_setting.name, env_setting.value);
            }
        }
    }

    // serialize the clusters back to XML
    let serialized = serde_xml_rs::to_string(&clusters)?;
    println!("Serialized XML:\n{}", serialized);

    Ok(())
}
