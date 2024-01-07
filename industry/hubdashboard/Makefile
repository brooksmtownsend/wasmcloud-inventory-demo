DEFAULT_GOAL := help

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

build: ## Build the project
	cd ui && npm run build
	@cosmo build

localrun: build ## Rebuild and rerun this actor
	@wash stop actor $(wash get hosts -o json | jq -r '.hosts[0].id') MB7AYIRSBBCYXP5RJQFH6DHV6DZEZCAZHPAWUTQ2XR3SFBXAZJ5OSIXK --count 0

run: build ## Run the actor on Cosmonic
	@cosmo launch

.PHONY: help build run
