export DATABASE_URL="sqlite://data/database.db" && 
sqlx migrate add -r $1 &&
unset DATABASE_URL