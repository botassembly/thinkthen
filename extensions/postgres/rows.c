#include "pg_internal.h"
#include "utils/builtins.h"

void tt_emit(TtOwner *owner, ReturnSetInfo *result)
{
    Datum values[7] = {0};
    bool nulls[7] = {false};
    const TtJson *field;
    const char *value;
    int code;
    TT_BRIDGE(code = thinkthen_row_columns(&owner->row, &owner->columns, &owner->error));
    tt_check(owner, code);
    owner->projection = tt_json_parse((char *)owner->columns.data, owner->columns.len);
    if (owner->projection == NULL)
        ereport(ERROR, (errcode(ERRCODE_OUT_OF_MEMORY), errmsg("cannot decode engine row projection")));
    values[0] = Int64GetDatum(owner->row.position);
    value = tt_json_string(tt_json_get(owner->projection, "status"));
    values[1] = PointerGetDatum(tt_server_text(value, strlen(value)));
    field = tt_json_get(owner->projection, "answer");
    nulls[2] = field == NULL;
    if (field != NULL) {
        value = tt_json_string(field);
        values[2] = PointerGetDatum(tt_server_text(value, tt_json_length(field)));
    }
    field = tt_json_get(owner->projection, "p");
    nulls[3] = field == NULL;
    if (field != NULL)
        values[3] = Float8GetDatum(tt_json_number(field));
    field = tt_json_get(owner->projection, "error");
    nulls[4] = field == NULL;
    if (field != NULL) {
        value = tt_json_string(field);
        values[4] = PointerGetDatum(tt_server_text(value, tt_json_length(field)));
    }
    nulls[5] = owner->row.detail.data == NULL;
    if (!nulls[5])
        values[5] = PointerGetDatum(tt_server_text((char *)owner->row.detail.data, owner->row.detail.len));
    values[6] = BoolGetDatum(owner->row.kept != 0);
    tuplestore_putvalues(result->setResult, result->setDesc, values, nulls);
    tt_json_free(owner->projection);
    owner->projection = NULL;
    TT_BRIDGE({ thinkthen_buffer_free(&owner->columns); thinkthen_row_free(&owner->row); });
}
