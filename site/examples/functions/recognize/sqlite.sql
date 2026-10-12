.load ./thinkthen

SELECT name.text, name.kind
FROM thinkthen_recognize(
    'Sam Patel says his Kindle arrived broken ' ||
    'in Denver.',
    'person,product,place'
) AS name;
