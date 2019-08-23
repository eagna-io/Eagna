deploy:
	cd frontend && yarn build
	firebase deploy
