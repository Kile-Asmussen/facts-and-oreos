
FACTORIO=~/.steam/steam/steamapps/common/Factorio/data

.PHONY: download-api-json

download-api:
	mkdir -p api
	wget -O api/archive.zip https://lua-api.factorio.com/latest/static/archive.zip
	unzip api/archive.zip -d api
	find api/files -type f -name '*.html' -print0 | parallel -q -0 -j8 pandoc -w plain {} -o {.}.txt >/dev/null
	find api/files -type f -name '*.html' -exec rm {} \;
	find api/files -type f -name '*.css' -exec rm {} \;
	find api/files -type f -name '*.js' -exec rm {} \;
	rm -rf api/files/static
	mv api/files/* api/
	rmdir api/files
	rm api/archive.zip

download-flua:
	git clone https://github.com/Rseding91/Factorio-Lua.git flua
	rm -rf flua/.git*

download-mlua:
	git clone https://github.com/mlua-rs/mlua.git mlua
	rm -rf flua/.git*

download-factorio-tools:
	git clone https://github.com/MForster/factorio-rust-tools.git ftools
	rm -rf ftools/.git*

get-factorio-references:
	mkdir -p .factorio/vanilla
	cp -r ${FACTORIO}/core ./.factorio/vanilla
	cp -r ${FACTORIO}/base ./.factorio/vanilla
	cp -r ${FACTORIO}/recycler ./.factorio/vanilla
	cp -r ${FACTORIO}/elevated-rails ./.factorio/vanilla
	cp -r ${FACTORIO}/space-age ./.factorio/vanilla
	cp -r ${FACTORIO}/quality ./.factorio/vanilla