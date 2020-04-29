use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error;
use tokio_postgres::row::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleProvince {
    id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleCity{
    id: u32,
    name: String, 
    pid: u32,
    province: String,
    lgt: f32,
    lat: f32,
}


impl SimpleCity{
    fn from(row: &Row) -> Self {
        let id: i32 = row.get("city_id");
        let name: String = row.get("city");
        let province_id: i32 = row.get("province_id");
        let province: String = row.get("province");
        let lgt: f32 = row.get("longitude");
        let lat: f32 = row.get("latitude");
        Self{
            id: id as u32, name, pid: province_id as u32, province, lgt, lat,
        }
    }
}

pub async fn list_all_simple_provinces(client: &Client) -> Result<Vec<SimpleProvince>, Error> {
    //todo!()
    let mut simple_privinces: Vec<SimpleProvince> = Vec::new();
    let rows = 
        client.query("SELECT DISTINCT province_id, province FROM public.base_region WHERE level = 1 ORDER BY province_id",&[])
        .await?;
    for row in &rows{
        let id: i32 = row.get("province_id");
        let name: String = row.get("province");
        simple_privinces.push(SimpleProvince{id: id as u32, name,})
    }
    Ok(simple_privinces)
}


pub async fn list_cities_by_province_id(client: & Client, province_id: i32) -> Result<Vec<SimpleCity>, Error>{

    // first style
    // let rows: Vec<Row> = 
    //     client.query("SELECT DISTINCT city_id, city, province_id, province, longitude, latitude FROM public.base_region WHERE province_id = $1 AND level = 2 ORDER BY city_id", &[&province_id])
    //     .await?;
    // let mut cities: Vec<SimpleCity> = Vec::new();
    // for row in &rows{
    //     cities.push(SimpleCity::from(row));
    // }
    // Ok(cities)
    
    // second style
    // let rows = 
    //     client.query("SELECT DISTINCT city_id, city, province_id, province, longitude, latitude FROM public.base_region WHERE province_id = $1 AND level = 2 ORDER BY city_id", &[&province_id])
    //     .await?;
    // let cities = rows.iter().map(|row| SimpleCity::from(row)).collect::<Vec<SimpleCity>>();
    // Ok(cities)

    let rows: Vec<Row> = client
        .query("SELECT DISTINCT city_id, city, province_id, province, longitude, latitude FROM public.base_region WHERE province_id = $1 AND level = 2 ORDER BY city_id", &[&province_id])
        .await?;
    
    Ok(rows.iter().map(|row| SimpleCity::from(row)).collect::<Vec<SimpleCity>>())

    // third style
    // match client.query("SELECT DISTINCT city_id, city, province_id, province, longitude, latitude FROM public.base_region WHERE province_id = $1 AND level = 2 ORDER BY city_id", &[&province_id]).await{
    //     Err(why) => Err(why),
    //     Ok(rows) => Ok(rows.iter().map(|row| SimpleCity::from(row)).collect::<Vec<SimpleCity>>()),
    // }

    // i think `?` style maybe more geek than `match` style
}