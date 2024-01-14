FROM rust:1.75

WORKDIR /restaurant
COPY . .
RUN rm -f .env
RUN cargo install --path .
CMD ["restaurant-server"]