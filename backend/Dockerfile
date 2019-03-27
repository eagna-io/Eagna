FROM python:3.7.2-stretch

WORKDIR /home/rohanmarket

COPY src ./src

RUN pip install psycopg2 psycopg2-binary falcon

CMD ["python", "/home/rohanmarket/src/main.py"]
