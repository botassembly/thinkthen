.load ./thinkthen

WITH picked AS (
    SELECT thinkthen_find(
        'Which line gives the refund deadline?',
        json_array(
            'Returns need the original receipt.',
            'Refunds are issued within 30 days ' ||
            'of purchase.',
            'Shipping is free on orders over $50.',
            'Gift cards cannot be exchanged for cash.'
        )
    ) AS deadline_result
)
SELECT json_extract(deadline_result, '$.value')
    AS refund_deadline
FROM picked;
