CREATE TABLE rules(id INTEGER, body TEXT);
INSERT INTO rules VALUES
    (1, 'Book economy class for every flight ' ||
        'under six hours.'),
    (2, 'Submit receipts within 30 days of the trip.'),
    (3, 'Hotel stays are capped at 200 dollars a night.'),
    (4, 'Employees may book business class on any flight.'),
    (5, 'Rental cars need a manager''s approval.'),
    (6, 'Receipts may be submitted at any time, ' ||
        'with no deadline.'),
    (7, 'Meals are reimbursed up to 60 dollars a day.'),
    (8, 'Use the company travel portal for all bookings.');

SELECT * FROM thinkthen_relate(
    'rules',
    'id',
    'body',
    'either:contradicts'
) AS contradiction;
