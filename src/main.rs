#![feature(proc_macro_hygiene, decl_macro)]
#![allow(nonstandard_style)]


use rocket_contrib::json::Json;
use serde_json::Value;
use serde_json::json;
use rand::Rng;



#[macro_use]
extern crate rocket;






#[get("/probabilityOfUnitInjectorFail/<vin_number>")]
 fn get_fail_probability(vin_number: String) ->  Json<Value>{

            if vin_number.len() > 17 {
                Json(json!({
                    "error": "VIN number too long"
                }))
            }else if vin_number.len() < 17{
                Json(json!({
                    "error": "VIN number too short"
                }))
            }else {
                let mut rng = rand::thread_rng();
                let random_number: f64 = rng.gen_range(0.00, 1.00);
    
                let probability = (random_number * 100.0).round() / 100.0;

               Json(json!({
                   "failProbability": probability,
               }))   
            }
        }


#[get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
    fn get_diesel_usage(distance: f64, yearOfProduction: i32, fuelUsagePer100KM: f64) -> Json<Value> {

                    if distance < 0.00{
                        Json(json!({
                            "error": "Distance must be equal or greater than 0."
                        }))
                    }else if !(yearOfProduction >= 1995 && yearOfProduction <= 2015){
                        Json(json!({
                            "error": "Year of production must be between 1995 and 2015."
                        }))
                    }else if fuelUsagePer100KM < 0.00{
                        Json(json!({
                            "error": "Fuel usage must be greater than 0."
                        }))
                    }else{
                        let diesel_usage: f64 = (distance / 100.00) * fuelUsagePer100KM;
                        let diesel_usage_formatted = (diesel_usage * 100.0).round() / 100.0;
                        Json(json!({
                            "fuelConsumption": diesel_usage_formatted
                        }))
                    }
                }
            
fn main() {
        rocket::ignite()
        .mount("/", routes![get_fail_probability, get_diesel_usage]).launch();
}
