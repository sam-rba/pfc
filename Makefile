include config.mk

build:
	go mod tidy
	go build
	go test
	gofmt -l -s -w .

run: build
	./pfc

clean:
	rm -f pfc

install:
	go build -buildvcs=false
	cp pfc ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/pfc
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	cat doc/{intro.1,cmd.1,op.1,func.1,const.1} | \
		sed "s/VERSION/${VERSION}/g" > \
		${DESTDIR}${MANPREFIX}/man1/pfc.1
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/pfc.1

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/pfc \
		${DESTDIR}${MANPREFIX}/man1/pfc.1

.PHONY: build run install uninstall
