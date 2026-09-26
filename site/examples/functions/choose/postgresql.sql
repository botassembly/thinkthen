SELECT body, thinkthen_choose(
    '{"choose": "Which team owns this?",
      "options": {
        "billing": "Invoices, fees, and refunds.",
        "shipping": "Parcels and delivery.",
        "account": "Logins and passwords."
      },
      "threshold": 0.9}',
    body, NULL) AS team
FROM (VALUES
    ('Please refund the extra fee on my invoice.'),
    ('My parcel went to the wrong address.'),
    ('I cannot reset my password.'),
    ('My parcel never came, and now ' ||
     'I cannot log in to track it.')
) AS t(body);
