#!/bin/bash

psql -U postgres -h 127.0.0.1 -f setup_db_and_user.sql
