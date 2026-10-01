SELECT thinkthen_choose(
    '{"choose": "Which team owns this?",
      "options": {
        "billing": "Invoices, fees, and refunds.",
        "shipping": "Parcels and delivery.",
        "account": "Logins and passwords."
      }}',
    'My parcel went to the wrong address.',
    NULL
) AS team;
