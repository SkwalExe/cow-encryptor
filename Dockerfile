FROM rust:latest
WORKDIR /app
COPY . /app
LABEL maintainer="Léopold Koprivnik Ibghy <skwal.net@gmail.com>"
RUN cargo build --release
RUN cp target/release/cow-encryptor /usr/local/bin/cow-encryptor
RUN chmod +x /usr/local/bin/cow-encryptor
RUN git clone https://github.com/SkwalExe/cow-translator.git
RUN cd cow-translator && RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build --release && cp target/release/cow-translator /usr/local/bin/cow-translator && chmod +x /usr/local/bin/cow-translator
ENTRYPOINT ["bash"]
