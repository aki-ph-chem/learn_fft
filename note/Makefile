TEX = platex
PDF = dvipdfmx
f = main

viewer = qpdfview
#viewer = mupdf

${f}.pdf : ${f}.dvi
	${PDF} $<

#${f}.dvi : ${f}.tex
#	${TEX} $< 
${f}.dvi : ${f}.tex src/*.tex
	@(${TEX} -interaction=nonstopmode $< > /dev/null 2>&1); \
		if [ $$? -eq 0 ]; then \
		echo "compile 1 is successed!"; \
		else \
		echo -e "failure! please read ${f}.log"; \
                exit 1;\
		fi
	@((grep -q "Rerun to get" ${f}.log || [ -f ${f}.toc ]) && platex -interaction=nonstopmode $< > /dev/null 2>&1); \
		if [ $$? -eq 0 ]; then \
		echo "compile 2 is successed!"; \
		fi

opn :
	${viewer} ${f}.pdf &

clean: 
	rm *.aux *.dvi *.log *.pdf *.toc *.out

.PHONY : opn clean
        
