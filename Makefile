include config.mk

build:
	go mod tidy
	go build
	go test
	gofmt -l -s -w .

clean:
	rm -f pfc

install:
	go build
	cp pfc ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/pfc
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	sed "s/VERSION/${VERSION}/g" < pfc.1 > ${DESTDIR}${MANPREFIX}/man1/pfc.1
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/pfc.1

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/pfc \
		${DESTDIR}${MANPREFIX}/man1/pfc.1

.PHONY: build install uninstall
