FROM python:3.9-rc-buster

# Create app directory
WORKDIR /usr/src/app

# Install app
COPY requirements.txt ./

RUN pip install --upgrade pip setuptools && \
    pip install --no-cache-dir --upgrade -r requirements.txt -t .

COPY app.py ./

EXPOSE 49163

ENTRYPOINT ["python", "app.py", "-s", "0.0.0.0"]