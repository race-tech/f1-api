clippy:
	cargo clippy --all-targets -- -Dwarnings

db: f1db.sql.gz
	docker build -f resources/dockerfiles/database/Dockerfile -t ghcr.io/race-tech/f1-db:latest .
