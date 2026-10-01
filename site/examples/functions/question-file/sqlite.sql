.load ./thinkthen

SELECT thinkthen_decide(
    '@refund.json',
    'I would like to return this and get ' ||
    'my money back.' || char(10)
) AS is_refund;
