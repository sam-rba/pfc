include config.mk

build:
	cargo fmt --all
	cargo test
	cargo build

run: build
	target/debug/pfc

doc:
	cargo doc --open

clean:
	rm -r target

install:
	cargo build --release
	cp target/release/pfc ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/pfc
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	sed "s/VERSION/${VERSION}/g" < pfc.1 > ${DESTDIR}${MANPREFIX}/man1/pfc.1
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/pfc.1

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/pfc\
		${DESTDIR}${MANPREFIX}/man1/pfc.1

.PHONY: build run doc clean install uninstall
