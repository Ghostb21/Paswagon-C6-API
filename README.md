# Paswagon-C6-API

## How to start the project?
1) Download and unzip code files,
2) use the `cargo build` command inside the folder,
3) use the `cargo run` command and voila - tons of possibilities right at Your fingertips.

## To be exact, 2 possibilities. This API contains only 2 GET endpoints:

### `/calculateDieselUsageForDistance`
Calculates input in the URL request and returns a number - liters of used diesel per distance.

  It takes 3 query parameters: 
  * distance (in kilometers),
  * yearOfProduction (1995 - 2015),
  * fuelUsagePer100KM.
    
    
    
  Example of URL request: <br/>
  `/calculateDieselUsageForDistance?distance=21&yearOfProduction=2015&fuelUsagePer100KM=10` returns `{"fuelUsage":2.1}`
  
### `/probabilityOfUnitInjectorFail`
Returns the probability of unit injector fail calculated on the basis of a totally meaningless VIN number. Vary from 0.00 (0%) to 1.00 (100%).

  It takes 1 query parameter:
  * VIN (needs to contain **exactly** 17 characters)
    
   Example of URL request:<br/>
   `/probabilityOfUnitInjectorFail?VIN=4Y1SL65848Z411439` returns `{"failProbability":0.79}`
  

If the value of any parameter will be invalid, it should return JSON with an error.
Example: <br/>
`{"error":"Distance must be equal or greater than 0."}`
