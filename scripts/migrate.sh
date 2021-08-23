export DATABASE_URL="sqlite://data/database.db" && 
sqlx migrate run &&
unset DATABASE_URL