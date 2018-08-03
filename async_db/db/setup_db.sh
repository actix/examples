#!/usr/bin/env bash
sqlite3 weather.db < db.sql
sqlite3 -csv weather.db ".import nyc_centralpark_weather.csv nyc_weather"
