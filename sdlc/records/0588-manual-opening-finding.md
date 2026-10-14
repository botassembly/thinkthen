# 0588: Record the approved manual workbook check

Ticket: 0588
Date: 2026-10-14

Ian opened three experiment outputs in Google Sheets: `fixture-00.xlsm` (macro, chart and pivot), `fixture-03.xlsx` (chart and merged cells), and `fixture-30-other-sheet-table.xlsx` (a table on the untouched sheet). All three looked right. His ruling in the ticket accepts this check in place of Excel for the web for this experiment. The writer passes this experiment, and spreadsheets remain in 0.3.

This adds Ian's manual observation to the previously reviewed Rust, ZIP preservation, Open XML SDK and LibreOffice findings. It does not claim Excel validation. The product writer must be opened in Excel for the web before the 0.3 release.

The pending manual-check and retention instructions in the earlier experiment record are superseded by this ruling. The retained folder `experiments/484-spreadsheet-writer` holds generated comparison workbooks, output workbooks and their ZIP, scripts, licenses, raw logs and results. Its reviewed findings and manual result are now recorded in the repository; the folder can be removed after the record-only landing. No product code or release work changes, no model calls run, and nothing is published.
