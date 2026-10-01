.load ./thinkthen

SELECT thinkthen_score(
    '{"score": "How urgent is this?",
      "levels": ["Routine.", "Soon.", "Immediate."]}',
    'Our checkout page is down and customers ' ||
    'cannot pay.' || char(10)
) AS urgency;
