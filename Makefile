
FACTORIO=~/.steam/steam/steamapps/common/Factorio/data

.PHONY: all clean submodules

all: ./reference/api ./reference/flua ./reference/mlua ./reference/ftools ./reference/trio ./reference/flua-src-rs ./.factorio/vanilla

submodules:
	git submodule add https://github.com/Rseding91/Factorio-Lua.git flua-src/flua
	git submodule update --recursive

clean:
	rm -rf ./.factorio/vanilla ./reference

./reference/api:
	mkdir -p ./reference/api
	wget -O ./reference/api/archive.zip https://lua-api.factorio.com/latest/static/archive.zip
	unzip ./reference/api/archive.zip -d api
	find api/files -type f -name '*.html' -print0 | parallel -q -0 -j8 pandoc -w plain {} -o {.}.txt >/dev/null
	find api/files -type f -name '*.html' -exec rm {} \;
	find api/files -type f -name '*.css' -exec rm {} \;
	find api/files -type f -name '*.js' -exec rm {} \;
	rm -rf ./reference/api/files/static
	mv./reference/api/files/* ./reference/api/
	rmdir ./reference/api/files

./reference/flua:
	mkdir -p ./reference/flua
	git clone https://github.com/Rseding91/Factorio-Lua.git ./reference/flua

./reference/mlua:
	mkdir -p ./reference/mlua
	git clone https://github.com/mlua-rs/mlua.git ./reference/mlua

./reference/ftools:
	mkdir -p ./reference/ftools
	git clone https://github.com/MForster/factorio-rust-tools.git ./reference/ftools

./reference/flua-src-rs:
	mkdir -p ./reference/flua-src-rs
	git clone https://github.com/fgardt/flua-src-rs ./reference/flua-src-rs

./reference/.factorio/vanilla:
	mkdir -p .factorio/vanilla
	cp -r ${FACTORIO}/core ./.factorio/vanilla
	cp -r ${FACTORIO}/base ./.factorio/vanilla
	cp -r ${FACTORIO}/recycler ./.factorio/vanilla
	cp -r ${FACTORIO}/elevated-rails ./.factorio/vanilla
	cp -r ${FACTORIO}/space-age ./.factorio/vanilla
	cp -r ${FACTORIO}/quality ./.factorio/vanilla