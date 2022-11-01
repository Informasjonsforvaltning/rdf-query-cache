run: dockerup rustrun dockerdown
test: dockerup rusttest dockerdown

rusttest:
	rm -rf ./tmp-repos
	GIT_REPOS_ROOT_PATH=./tmp-repos \
	GIT_REPO_URL=http://gitea:gitea123@localhost:3000/gitea/diff-store.git \
		cargo test

rustrun:
	GIT_REPOS_ROOT_PATH=./tmp-repos
	GIT_REPO_URL=http://gitea:gitea123@localhost:3000/gitea/diff-store.git \
		cargo run

dockerup:
	docker-compose up -d

dockerdown:
	docker-compose down -v
