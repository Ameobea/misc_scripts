FROM python:3.12.3-slim-bullseye

WORKDIR /usr/src/app

COPY requirements.txt ./
RUN apt update && apt install -y curl && pip install --no-cache-dir -r requirements.txt

COPY . .

CMD [ "python", "main.py" ]
