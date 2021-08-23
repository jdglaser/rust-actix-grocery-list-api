export DATABASE_URL="sqlite://data/database.db" && 
sqlx migrate revert &&
unset DATABASE_URL