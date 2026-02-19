FROM python:3.14-trixie

WORKDIR /app

ENV ENV_TEST="env works!"

COPY copy_test.txt copy_test.txt
COPY copy-source cp

RUN echo 'build works!' > build_test.txt