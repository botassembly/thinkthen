thinkthen choose --column answer --into sentiment \
  --choice positive --choice negative --choice neutral \
  --threshold 0.7 \
  'What is the tone of this survey answer?' survey.csv
cut -d, -f1-5 survey.thinkthen.csv
