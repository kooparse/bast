install:
	cargo build && \
	(cd ./website && npm i)

dev: 
	cargo watch -i "website/**/*" -x run & \
	(cd ./website/ && npm run dev)

build_server:
	cargo build --release

build_front: 
	(cd ./website/ && npm i --production && npm run export) && \
	cp -R ./website/out/ ./static/front/
