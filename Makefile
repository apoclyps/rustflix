.PHONY: help start db.console network


default: help

db.console: network ## Open a database console
	@docker-compose exec db psql -h localhost -U postgres development

help: ## show this help
	@echo
	@fgrep -h " ## " $(MAKEFILE_LIST) | fgrep -v fgrep | sed -Ee 's/([a-z.]*):[^#]*##(.*)/\1##\2/' | column -t -s "##"
	@echo

start: ## run the application locally in the background
	@docker-compose up -d


network: ## Create the rustflix network if it doesn't exist
	docker network create --driver bridge rustflix || true
