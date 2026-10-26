EXTENSION = thinkthen
MODULE_big = thinkthen_postgres
OBJS = boundary.o source.o extension.o question.o rows.o $(THINKTHEN_TEST_ROUTE_OBJECT)
DATA = thinkthen--0.2.0.sql
PG_CONFIG ?= pg_config
PG_CPPFLAGS = -I../../crates/thinkthen-ffi/include
PG_CFLAGS = -std=c11 -Wall -Wextra -Werror -Wno-unused-parameter $(THINKTHEN_PG_CFLAGS)
SHLIB_LINK = $(THINKTHEN_ARCHIVE) -Wl,--exclude-libs,ALL -ldl -lpthread -lm -lrt -lutil $(THINKTHEN_PG_CFLAGS)
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
