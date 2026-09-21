-- the question reads like any other condition
SELECT id, body
FROM 'tickets.parquet'
WHERE thinkthen_decide('Is this a complaint?', body);

-- one option per row, most urgent rows first
SELECT id,
    thinkthen_choose('Which team owns this?', body,
        ['billing', 'shipping', 'account']) AS team
FROM 'tickets.parquet'
ORDER BY thinkthen_score('How urgent?', body,
    ['Routine.', 'Soon.', 'Immediate.']) DESC;
