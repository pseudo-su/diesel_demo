.PHONY: devstack.start
devstack.start:
	docker-compose up -d --remove-orphans devstack

.PHONY: devstack.stop
devstack.stop:
	docker-compose down --remove-orphans

.PHONY: devstack.clean
devstack.clean:
	rm -rf ./devstack/postgres/data/*
	docker-compose rm --stop --force

.PHONY: devstack.restart
devstack.restart: devstack.stop devstack.start

.PHONY: devstack.recreate
devstack.recreate: devstack.clean devstack.restart
