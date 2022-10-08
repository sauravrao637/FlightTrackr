use std::collections::HashSet;

use handle_errors::Error;

use crate::types::flights::Flights;

pub async fn calculate(flights: Flights) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Starting to calculate");
    let start_set: &mut HashSet<String> = &mut HashSet::new();
    let end_set: &mut HashSet<String> = &mut HashSet::new();

    for flight in flights.0 {
        if end_set.contains(&flight.starting.0) {
            end_set.remove(&flight.starting.0);
        } else {
            start_set.insert(flight.starting.0);
        }

        if start_set.contains(&flight.ending.0) {
            start_set.remove(&flight.ending.0);
        } else {
            end_set.insert(flight.ending.0);
        }
    }
    if start_set.len() != 1 {
        log::error!("Invalid Path Formation");
        return Err(warp::reject::custom(Error::InvalidFlightPath));
    }
    let reply = vec![
        start_set.iter().next().unwrap().to_owned(),
        end_set.iter().next().unwrap().to_owned(),
    ];
    Ok(warp::reply::json(&reply))
}
