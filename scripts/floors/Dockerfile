# Online preparation only. Product builds use this image without a network.
FROM quay.io/pypa/manylinux_2_28_x86_64@sha256:076b19ef6e698f28649fa817c49d14443201e01da85240e8e52f93e8daa49bff
RUN dnf install -y readline-devel bzip2-devel xz-devel pcre2-devel libcurl-devel libicu-devel libxml2-devel libedit-devel && dnf clean all
COPY scripts/release_downloads.py scripts/release-downloads.json /opt/thinkthen-downloads/
WORKDIR /tmp/sources
RUN /opt/python/cp313-cp313/bin/python3 /opt/thinkthen-downloads/release_downloads.py https://nodejs.org/dist/v22.12.0/node-v22.12.0-linux-x64.tar.xz node.tar.xz \
 && mkdir /opt/node && tar -xf node.tar.xz -C /opt/node --strip-components=1 \
 && /opt/python/cp313-cp313/bin/python3 /opt/thinkthen-downloads/release_downloads.py https://go.dev/dl/go1.23.12.linux-amd64.tar.gz go.tar.gz \
 && tar -xf go.tar.gz -C /opt
RUN /opt/python/cp313-cp313/bin/python3 /opt/thinkthen-downloads/release_downloads.py https://cran.r-project.org/src/base/R-4/R-4.3.3.tar.gz R.tar.gz \
 && tar -xf R.tar.gz && cd R-4.3.3 \
 && ./configure --prefix=/opt/R --enable-R-shlib --with-x=no --with-readline=yes --without-recommended-packages \
 && make -j2 > /tmp/R-build.log 2>&1 && make install > /tmp/R-install.log 2>&1
RUN dnf install -y libyaml-devel openssl-devel && dnf clean all \
 && /opt/python/cp313-cp313/bin/python3 /opt/thinkthen-downloads/release_downloads.py https://cache.ruby-lang.org/pub/ruby/3.3/ruby-3.3.8.tar.gz ruby.tar.gz \
 && tar -xf ruby.tar.gz && cd ruby-3.3.8 \
 && ./configure --prefix=/opt/ruby --disable-install-doc \
 && make -j2 > /tmp/ruby-build.log 2>&1 && make install > /tmp/ruby-install.log 2>&1 \
 && /opt/ruby/bin/ruby -rpsych -rzlib -ropenssl -e 'puts RUBY_VERSION'
RUN dnf install -y flex mold && dnf clean all
RUN /opt/python/cp313-cp313/bin/python3 /opt/thinkthen-downloads/release_downloads.py https://ftp.postgresql.org/pub/source/v18.6/postgresql-18.6.tar.bz2 pg.tar.bz2 \
 && tar -xf pg.tar.bz2 && cd postgresql-18.6 \
 && ./configure --prefix=/opt/thinkthen-pg18 --without-icu --without-readline --without-zstd --without-lz4 \
 && make -j2 > /tmp/pg-build.log 2>&1 && make install > /tmp/pg-install.log 2>&1
RUN mkdir -p /opt/floor/sources && cp *.tar.* /opt/floor/sources/ \
 && sha256sum /opt/floor/sources/* > /opt/floor/sources/SHA256SUMS \
 && rm -rf /tmp/sources /tmp/*-build.log /tmp/*-install.log
ENV PATH=/opt/R/bin:/opt/ruby/bin:/opt/node/bin:/opt/go/bin:/opt/thinkthen-pg18/bin:/opt/python/cp313-cp313/bin:/opt/rh/gcc-toolset-14/root/usr/bin:/usr/local/bin:/usr/bin:/bin
WORKDIR /src
