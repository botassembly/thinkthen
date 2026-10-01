LOAD './thinkthen.duckdb_extension';

SELECT id, thinkthen_choose(
    '{"choose": "Which team owns this?",
      "options": {
        "billing": "Invoices, fees, and refunds.",
        "shipping": "Parcels and delivery.",
        "account": "Logins and passwords."
      },
      "threshold": 0.9}',
    body) AS team
FROM (VALUES
    (1, 'Please refund the extra fee on my invoice.'),
    (2, 'My parcel went to the wrong address.'),
    (3, 'I cannot reset my password.'),
    (4, 'My parcel never came, and now ' ||
        'I cannot log in to track it.')
) t(id, body);
