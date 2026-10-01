.load ./thinkthen

CREATE TABLE rules(id INTEGER, body TEXT, kind TEXT);
INSERT INTO rules VALUES
    (1, 'Book economy class for every flight ' ||
        'under six hours.', 'rule'),
    (2, 'Submit receipts within 30 days of the trip.',
        'rule'),
    (3, 'Hotel stays are capped at 200 dollars a night.',
        'rule'),
    (4, 'Employees may book business class on any flight.',
        'rule'),
    (5, 'Rental cars need a manager''s approval.', 'rule'),
    (6, 'Receipts may be submitted at any time, ' ||
        'with no deadline.', 'rule'),
    (7, 'Meals are reimbursed up to 60 dollars a day.',
        'rule'),
    (8, 'Use the company travel portal for all bookings.',
        'rule');

SELECT * FROM thinkthen_relate(
    'SELECT id, body AS name, kind FROM rules',
    'either:contradicts'
) AS contradiction;
