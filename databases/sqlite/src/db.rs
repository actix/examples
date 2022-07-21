use std::{thread::sleep, time::Duration};

use actix_web::{error, web, Error};
use rusqlite::Statement;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type WeatherAggResult = Result<Vec<WeatherAgg>, rusqlite::Error>;

#[derive(Debug, Serialize, Deserialize)]
pub enum WeatherAgg {
    AnnualAgg { year: i32, total: f64 },
    MonthAgg { year: i32, month: i32, total: f64 },
}

#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetTopTenHottestYears,
    GetTopTenColdestYears,
    GetTopTenHottestMonths,
    GetTopTenColdestMonths,
}

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<WeatherAgg>, Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        // simulate an expensive query, see comments at top of main.rs
        sleep(Duration::from_secs(1));

        match query {
            Queries::GetTopTenHottestYears => get_hottest_years(conn),
            Queries::GetTopTenColdestYears => get_coldest_years(conn),
            Queries::GetTopTenHottestMonths => get_hottest_months(conn),
            Queries::GetTopTenColdestMonths => get_coldest_months(conn),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

fn get_hottest_years(conn: Connection) -> WeatherAggResult {
    let stmt = conn.prepare(
        "
    SELECT cast(strftime('%Y', date) as int) as theyear,
            sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear
        ORDER BY total DESC LIMIT 10",
    )?;

    get_rows_as_annual_agg(stmt)
}

fn get_coldest_years(conn: Connection) -> WeatherAggResult {
    let stmt = conn.prepare(
        "
        SELECT cast(strftime('%Y', date) as int) as theyear,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear
        ORDER BY total ASC LIMIT 10",
    )?;

    get_rows_as_annual_agg(stmt)
}

fn get_rows_as_annual_agg(mut statement: Statement) -> WeatherAggResult {
    statement
        .query_map([], |row| {
            Ok(WeatherAgg::AnnualAgg {
                year: row.get(0)?,
                total: row.get(1)?,
            })
        })
        .and_then(Iterator::collect)
}

fn get_hottest_months(conn: Connection) -> WeatherAggResult {
    let stmt = conn.prepare(
        "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total DESC LIMIT 10",
    )?;

    get_rows_as_month_agg(stmt)
}

fn get_coldest_months(conn: Connection) -> WeatherAggResult {
    let stmt = conn.prepare(
        "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth,
                sum(tmax) as total
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total ASC LIMIT 10",
    )?;

    get_rows_as_month_agg(stmt)
}

fn get_rows_as_month_agg(mut statement: Statement) -> WeatherAggResult {
    statement
        .query_map([], |row| {
            Ok(WeatherAgg::MonthAgg {
                year: row.get(0)?,
                month: row.get(1)?,
                total: row.get(2)?,
            })
        })
        .and_then(Iterator::collect)
}
