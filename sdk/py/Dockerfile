FROM python:3.11-slim-bookworm as poetry

RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/* && \
    python3 -m pip install --user pipx
ENV PATH="/root/.local/bin:${PATH}"
RUN pipx install poetry && \
    poetry config virtualenvs.create false

WORKDIR /app
COPY . .
RUN poetry install --no-interaction && \
    poetry cache clear pypi --all && \
    chmod +x /app/wait-for-server.sh

ENV CLIENT_SECRET=/app/secrets/secret
ENTRYPOINT ["/app/wait-for-server.sh", "poetry", "run", "python", "-m", "splendor_actor"]