use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Station = Vec<Datapoint>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
    pub sensordatavalues: Vec<Sensordatavalue>,
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensordatavalue {
    #[serde(rename = "value_type")]
    pub value_type: String,
    pub value: Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _json = r#"
    [{
    "sensordatavalues": [
      {
        "value_type": "temperature",
        "value": "8.54"
      },
      {
        "value_type": "pressure",
        "value": "100102.09"
      },
      {
        "value_type": "humidity",
        "value": "95.13"
      }
    ],
    "timestamp": "2022-04-06 08:42:38"
    }]
    "#;
    let request_url = format!("https://data.sensor.community/airrohr/v1/sensor/{sensor}/",
                              sensor = "59497");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;
    let station: Station = response.json().await?;
    //let station: Station = serde_json::from_str(json).unwrap();
    println!("{:?}", station);
    Ok(())
}
