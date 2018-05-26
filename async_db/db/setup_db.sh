sqlite3 weather.db < db/db.sql ; \
sqlite3 -csv weather.db ".import db/nyc_centralpark_weather.csv nyc_weather"
