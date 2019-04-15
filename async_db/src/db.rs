use actix_web::{web, Error as AWError};
use failure::Error;
use futures::Future;
use r2d2;
use r2d2_sqlite;
use rusqlite::NO_PARAMS;
use serde_derive::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug, Serialize, Deserialize)]
pub enum WeatherAgg {
    AnnualAgg { year: i32, total: f64 },
    MonthAgg { year: i32, month: i32, total: f64 },
}

pub enum Queries {
    GetTopTenHottestYears,
    GetTopTenColdestYears,
    GetTopTenHottestMonths,
    GetTopTenColdestMonths,
}

pub fn execute(
    pool: &Pool,
    query: Queries,
) -> impl Future<Item = Vec<WeatherAgg>, Error = AWError> {
    let pool = pool.clone();
    web::block(move || match query {
        Queries::GetTopTenHottestYears => get_hottest_years(pool.get()?),
        Queries::GetTopTenColdestYears => get_coldest_years(pool.get()?),
        Queries::GetTopTenHottestMonths => get_hottest_months(pool.get()?),
        Queries::GetTopTenColdestMonths => get_coldest_months(pool.get()?),
    })
    .from_err()
}

fn get_hottest_years(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = "
    SELECT cast(strftime('%Y', date) as int) as theyear,
            sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear
        ORDER BY total DESC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| WeatherAgg::AnnualAgg {
            year: row.get(0),
            total: row.get(1),
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<WeatherAgg>>())
        })?;

    sleep(Duration::from_secs(2)); //see comments at top of main.rs

    Ok(annuals)
}

fn get_coldest_years(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = "
        SELECT cast(strftime('%Y', date) as int) as theyear,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear
        ORDER BY total ASC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| WeatherAgg::AnnualAgg {
            year: row.get(0),
            total: row.get(1),
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<WeatherAgg>>())
        })?;

    sleep(Duration::from_secs(2)); //see comments at top of main.rs

    Ok(annuals)
}

fn get_hottest_months(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total DESC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| WeatherAgg::MonthAgg {
            year: row.get(0),
            month: row.get(1),
            total: row.get(2),
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<WeatherAgg>>())
        })?;

    sleep(Duration::from_secs(2)); //see comments at top of main.rs
    Ok(annuals)
}

fn get_coldest_months(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total ASC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| WeatherAgg::MonthAgg {
            year: row.get(0),
            month: row.get(1),
            total: row.get(2),
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<WeatherAgg>>())
        })?;

    sleep(Duration::from_secs(2)); //see comments at top of main.rs
    Ok(annuals)
}
