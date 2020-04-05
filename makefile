install:
	cargo build && \
	(cd ./website && npm i)
dev: 
	cargo watch -i "website/**/*" -x run & \
	(cd ./website/ && npm run dev)
build: 
	(cd ./website/ && npm run export) && \
	cp -R ./website/out/ ./static/front/

