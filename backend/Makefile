.PHONY: print_schema
print_schema:
	cd crop_infra && make print_schema


.PHONY: test
test:
	bash tests/run-all.sh


.PHONY: deploy
deploy:
	heroku container:push web --arg BUILD_MODE=debug -a crop-instapoll-backend-staging
	heroku container:release web -a crop-instapoll-backend-staging
