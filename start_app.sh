echo "Initializing $0"

diesel setup --database-url='postgres://postgres:tree-db-dev@'$1'/treebook'
diesel migration run

cargo run