FROM python:3.7.2-stretch

WORKDIR /home/rohanmarket

RUN pip install psycopg2 psycopg2-binary falcon

COPY src ./src

CMD ["python", "/home/rohanmarket/src/main.py"]
