# SQLite Weather Data

This directory includes weather information obtained from NOAA for NYC Central Park: https://www.ncdc.noaa.gov/cdo-web/

## Usage

From the repository root, create the SQLite database:

```sh
cd databases/sqlite
bash db/setup_db.sh
```

This writes `weather.db` in `databases/sqlite`. See the [example README](../README.md) for server instructions.
