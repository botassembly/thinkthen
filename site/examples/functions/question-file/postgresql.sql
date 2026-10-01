SELECT thinkthen_decide(
    '@refund.json',
    'I would like to return this and get ' ||
    'my money back.' || chr(10)
) AS is_refund;
