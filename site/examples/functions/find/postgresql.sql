SELECT thinkthen_find(
    'Which line gives the refund deadline?',
    ARRAY[
        'Returns need the original receipt.',
        'Refunds are issued within 30 days ' ||
        'of purchase.',
        'Shipping is free on orders over $50.',
        'Gift cards cannot be exchanged for cash.'
    ]
) AS refund_deadline;
