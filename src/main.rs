use serde::{Deserialize, Serialize};

pub type Station = Vec<Datapoint>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dadapoint {
    pub sensordatavalues: Vec<Sensordatavalue>,
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensordatavalue {
    #[serde(rename = "value_type")]
    pub value_type: String,
    pub value: String,
}

fn main() {
    let json = r#"
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
    let station: Station = serde_json::from_str(json).unwrap();
    println!("{:?}", station);
}
