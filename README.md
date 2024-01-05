# Create docker container
docker run --name dev-postgres --rm -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=Pa55w0rd -e PGDATA=/var/lib/postgresql/data/pgdata -v /tmp:/var/lib/postgresql/data -P -p 127.0.0.1:5432:5432 -it postgres:alpine 

docker run --name dev-postgres --rm -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=Pa55w0rd -e PGDATA=/var/lib/postgresql/data/pgdata -v /tmp:/var/lib/postgresql/data -p 5432:5432 -it postgres:14.1-alpine

# Connect to docker container in interactive mode
docker exec -it dev-postgres psql -U doAdmin

# Grant all privileges on database to user

GRANT ALL PRIVILEGES ON DATABASE <<DATABASE_NAME>> TO <<USER>>

# Grant CRUD privileges to database user
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA <<SCHEMA_NAME>> TO <<USER>>;

# Grant Execute privileges to database user
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA <<SCHEMA_NAME>> TO <<USER>>;

# Run ite1 in the project root

cd tutor-db

cargo run --bin iter1

# Run the program from the workspace root instead  of the project root

cargo run --bin iter1 -p tutor-db

If you choose to do this make sure the .env file containing the database access credentials is located within the workspace root as opposed to the project root.