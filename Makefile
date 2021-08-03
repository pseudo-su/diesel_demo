.PHONY: deps.install
deps.install:
	cd cdk && npm install;

.PHONY: cdk.package
cdk.package:
	cd cdk && npx cdk synth;

.PHONY: cdk.deploy.dev
cdk.deploy.dev:
	cd cdk && npx cdk deploy --app=cdk.out 'Dev/*';

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
