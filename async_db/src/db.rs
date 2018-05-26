use actix::prelude::*;
use actix_web::*;
use std::{time::Duration, thread::sleep};
use failure::Error;
use r2d2;
use r2d2_sqlite;


pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;


pub struct DbExecutor(pub Pool);
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}


#[derive(Debug, Serialize, Deserialize)]
pub enum WeatherAgg {
    AnnualAgg {year: i32, total: f64},
    MonthAgg {year: i32, month: i32, total: f64}
}

pub enum Queries {
    GetTopTenHottestYears,
    GetTopTenColdestYears,
    GetTopTenHottestMonths,
    GetTopTenColdestMonths
}

//pub struct GetTopTenHottestYears;
impl Message for Queries {
    type Result = Result<Vec<WeatherAgg>, Error>;
}
impl Handler<Queries> for DbExecutor {
    type Result = Result<Vec<WeatherAgg>, Error>;

    fn handle(&mut self, msg: Queries, _: &mut Self::Context) -> Self::Result {
        let conn: Connection = self.0.get()?;

        match msg {
            Queries::GetTopTenHottestYears => get_hottest_years(conn),
            Queries::GetTopTenColdestYears => get_coldest_years(conn),
            Queries::GetTopTenHottestMonths =>get_hottest_months(conn),
            Queries::GetTopTenColdestMonths => get_coldest_months(conn),
        }
    }
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
    let annuals = prep_stmt.query_map(&[], |row| {
                            WeatherAgg::AnnualAgg{year: row.get(0),
                                                    total: row.get(1)}})
                        .and_then(|mapped_rows| 
        Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<WeatherAgg>>()))?;

    sleep(Duration::from_secs(2));

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
    let annuals = prep_stmt.query_map(&[], |row| {
                                WeatherAgg::AnnualAgg{year: row.get(0),
                                                    total: row.get(1)}})
                            .and_then(|mapped_rows| 
        Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<WeatherAgg>>()))?;

    sleep(Duration::from_secs(2));

    Ok(annuals)
}

fn get_hottest_months(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = 
        "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth, 
                sum(tmax) as total 
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total DESC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt.query_map(&[], |row| {
                                WeatherAgg::MonthAgg{year: row.get(0),
                                                    month: row.get(1),
                                                    total: row.get(2)}})
                            .and_then(|mapped_rows| 
        Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<WeatherAgg>>()))?;

    sleep(Duration::from_secs(2));
    Ok(annuals)
}

fn get_coldest_months(conn: Connection) -> Result<Vec<WeatherAgg>, Error> {
    let stmt = 
        "SELECT cast(strftime('%Y', date) as int) as theyear,
                cast(strftime('%m', date) as int) as themonth, 
                sum(tmax) as total 
        FROM nyc_weather
        WHERE tmax <> 'TMAX'
        GROUP BY theyear, themonth
        ORDER BY total ASC LIMIT 10;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt.query_map(&[], |row| {
                                WeatherAgg::MonthAgg{year: row.get(0),
                                                    month: row.get(1),
                                                    total: row.get(2)}})
                            .and_then(|mapped_rows| 
        Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<WeatherAgg>>()))?;

    sleep(Duration::from_secs(2));
    Ok(annuals)
}