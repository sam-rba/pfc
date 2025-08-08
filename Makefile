include config.mk

build:
	go mod tidy
	go build
	go test
	gofmt -l -s -w .

run: build
	./pfc

clean:
	rm -f pfc pfc-v*

doc:
	sed "s/VERSION/${VERSION}/g" < doc/intro.1 | \
		cat - doc/{cmd.1,op.1,func.1,const.1} > pfc.1

install: doc
	go build -buildvcs=false
	cp pfc ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/pfc
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	cp pfc.1 ${DESTDIR}${MANPREFIX}/man1
	chmod 644 ${DESTDIR}${MANPREFIX}/man1/pfc.1

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/pfc \
		${DESTDIR}${MANPREFIX}/man1/pfc.1

release:
	for os in ${GOOSES} ; do \
		for arch in ${GOARCHES} ; do \
			bin=pfc-v${VERSION}-$$os-$$arch; \
			if [ $$os = "windows" ] ; then bin=$$bin.exe; fi; \
			echo building $$bin; \
			GOOS=$$os GOARCH=$$arch go build -o $$bin; \
		done \
	done

.PHONY: build run clean doc install uninstall release
