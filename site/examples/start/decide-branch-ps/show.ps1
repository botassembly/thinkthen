'Please refund my order.' |
  thinkthen decide --check `
    'Does the customer request a refund?' > $null
if ($LASTEXITCODE -eq 0) {
  'send to the refunds queue'
}
