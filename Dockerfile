FROM ubuntu:focal


RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

# installing rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# installing gtk
RUN apt-get install libgtk-3-dev -y

RUN mkdir -p /user/address_book_gtk/
WORKDIR /user/address_book_gtk/
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]