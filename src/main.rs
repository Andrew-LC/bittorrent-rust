use serde_json::Value;
use std::env;
use serde_json::map::Map as SerdeMap;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
enum Bencode {
    String(String),
    Number(i64),
    List(Vec<Bencode>),
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value, ()> {
    match encoded_value.chars().next().unwrap() {
        'i' => {
            // Example: "i52e"
            let e_index = encoded_value.find('e').ok_or(())?;
            let string = &encoded_value[1..e_index];
            let number = string.parse::<serde_json::Number>().map_err(|_| ())?;

            Ok(Value::Number(number))
        },
        '0'..='9' => {
            // Example: "5:hello" -> "hello"
            if let Some((length, value)) = encoded_value.split_once(":") {
                let number = length.parse::<i64>().map_err(|_| ())?;
                let string = &value[0..number as usize];

                Ok(Value::String(string.to_string()))
            } else {
                Err(())
            }
        },
        'l' => {
            // Example: "l5:helloi52ee" -> ["hello", 52]
	    let mut list = Vec::new();
	    let mut index = 1;
	    let mut temp = &encoded_value[index..&encoded_value.len()-1];

	    while let Ok(value) = decode_bencoded_value(&temp) {
		if &temp.chars().nth(0).unwrap() == &'i' {
		    index += value.to_string().len() + 2;
		    temp = &encoded_value[index..encoded_value.len()];
		} else {
		    index += value.to_string().len();
		    temp = &encoded_value[index..encoded_value.len()];
		}
		list.push(value);
	    }

	    Ok(Value::Array(list))
	},
	'd' => {
	    // Example: {"hello": 52, "foo":"bar"} --> d3:foo3:bar5:helloi52ee
	    let mut map: SerdeMap<String, Value> = SerdeMap::new();
	    let mut stack: Vec<_> = vec![]; 
	    let mut index = 1;
	    let mut temp = &encoded_value[index..encoded_value.len()-1];

	    while let Ok(value) = decode_bencoded_value(&temp) {
		match stack.len() {
		    2 => {
			let key: String = stack.pop().unwrap();
			let val = stack.pop().unwrap();

			map.insert(key.to_string(), Value::String(val));
			stack = vec![];
		    },
		    _ => {
			if &temp.chars().nth(0).unwrap() == &'i' {
			    index += value.to_string().len() + 2;
			    temp = &encoded_value[index..encoded_value.len()];
			    println!("temp: {} value: {}", temp, value);
			} else {
			    index += value.to_string().len();
			    temp = &encoded_value[index..encoded_value.len()];
			    println!("temp: {} value: {}", temp, value);
			}
		    }
		}

		stack.push(value.to_string());
	    }

	    Ok(Value::Object(map))
	},
        _ => Err(()),
    }
}



// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.unwrap().to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
