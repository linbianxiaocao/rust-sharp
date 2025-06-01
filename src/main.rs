
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, io::Write};

use serde::{Deserialize, Serialize};
use csv::Reader;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServiceFabricClusters {
    #[serde(rename = "ServiceFabricCluster")]
    clusters: Vec<ServiceFabricCluster>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServiceFabricCluster {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "GlobalValues")]
    global_values: GlobalValues,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GlobalValues {
    #[serde(rename = "EnvironmentSetting")]
    env_settings: Vec<EnvironmentSetting>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    // for cluster in &clusters.clusters {
    //     for env_setting in &cluster.global_values.env_settings {
    //         if env_setting.name == "DstsApplicationPrincipalId" {
    //             println!("{}, {}: {}", cluster.name, env_setting.name, env_setting.value);
    //         }
    //     }
    // }

    let csv_file = File::open("DstsClientApplicationPrincipalId.csv")?;
    let mut rdr = Reader::from_reader(csv_file);

    let mut dict = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let region = record.get(0).unwrap();
        let dsts_id = record.get(1).unwrap();
        // println!("{:?},{:?}", region, dsts_id);
        dict.insert(region.to_string(), dsts_id.to_string());
    }

    // Clone the clusters to modify them
    let mut new_clusters = clusters.clone();

    for cluster in &clusters.clusters {
        let envs = &cluster.global_values.env_settings;

        // insert to the vector of env_settings right after the line with "DstsApplicationPrincipalId"
        for (i, env_setting) in envs.iter().enumerate() {
            if env_setting.name == "DstsApplicationPrincipalId" {
                if let Some(dsts_id) = dict.get(&cluster.name) {
                    // Insert the new setting right after the existing one
                    let mut new_envs = envs.to_vec();
                    new_envs.insert(i + 1, EnvironmentSetting {
                        name: "DstsClientApplicationPrincipalId".to_string(),
                        value: dsts_id.clone(),
                    });

                    // Update the cluster's global values with the new env settings
                    let mut updated_cluster = cluster.clone();
                    updated_cluster.global_values.env_settings = new_envs;

                    // Replace the same cluster in the cloned new_clusters
                    if let Some(existing_cluster) = new_clusters.clusters.iter_mut().find(|c| c.name == cluster.name) {
                        *existing_cluster = updated_cluster;
                    } else {
                        new_clusters.clusters.push(updated_cluster);
                    }
                }
            }
        }
    }

    // serialize the clusters back to XML
    let serialized = serde_xml_rs::to_string(&new_clusters)?;
    // println!("Serialized XML:\n{}", serialized);

    // append a new line after each '>' character in the serialized XML
    let serialized = serialized.replace(">", ">\n");

    // save the serialized XML to a file
    let mut output_file = File::create("output.xml")?;

    output_file.write_all(serialized.as_bytes())?;

    Ok(())
}
