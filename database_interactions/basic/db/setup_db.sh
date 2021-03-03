#!/usr/bin/env bash
cd $(dirname "$0")
sqlite3 ../weather.db < db.sql
sqlite3 -csv ../weather.db ".import nyc_centralpark_weather.csv nyc_weather"
