FROM rust:1.72-buster

WORKDIR /code

RUN apt-get install curl gnupg -y
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install nodejs -y

RUN git clone https://github.com/vim/vim.git
WORKDIR vim
RUN ./configure --with-features=huge
RUN make
RUN make install

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y -q \
    libssl-dev \
    libpq-dev \
    pkg-config \
    curl \
    build-essential \
    libdbus-1-dev \
    software-properties-common


RUN rustup toolchain install nightly --component rust-analyzer-preview
RUN rustup component add rls rust-analysis rust-src rustfmt
RUN curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim

COPY src /code/
COPY .vim /root/.vim/
COPY .vimrc /root/.vimrc

WORKDIR /code

