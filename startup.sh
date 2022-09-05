echo $DATABASE_URL > .env
diesel migration run
./target/release/todo-app