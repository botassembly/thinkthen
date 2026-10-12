#!/usr/bin/env node
// Edge cases for the same-case check in check-cases.mjs. Each row gives a
// sample's text and the strings of the case the check must find missing.

import { missing } from './check-cases.mjs';

const one = {
  question: ['Which team owns this?'],
  choices: ['billing', 'Parcels and delivery.'],
  input: ['My parcel went to the wrong address.'],
};
const all = [...one.question, ...one.choices, ...one.input];

const CASES = [
  ['every string on one line', 'q = "Which team owns this?"\nc = ["billing", "Parcels and delivery."]\nt = "My parcel went to the wrong address."', []],
  ['an empty sample', '', all],
  ['another question', 'q = "Who owns this?"\nc = ["billing", "Parcels and delivery."]\nt = "My parcel went to the wrong address."', ['Which team owns this?']],
  ['another choice', 'q = "Which team owns this?"\nc = ["refunds", "Parcels and delivery."]\nt = "My parcel went to the wrong address."', ['billing']],
  ['another input', 'q = "Which team owns this?"\nc = ["billing", "Parcels and delivery."]\nt = "My parcel never came."', ['My parcel went to the wrong address.']],
  ['a Python split', 'q = "Which team owns this?" "billing" "Parcels and delivery."\nt = (\n    "My parcel went "\n    "to the wrong address."\n)', []],
  ['a Java split', 'q = "Which team owns this?" "billing" "Parcels and delivery."\nt = "My parcel went to the "\n    + "wrong address.";', []],
  ['an SQL split', "'Which team owns this?' 'billing' 'Parcels and delivery.'\n'My parcel went to ' ||\n    'the wrong address.'", []],
  ['a Bash split', 'q="Which team owns this?" billing "Parcels and delivery."\nt="My parcel went to the"\nt+=" wrong address."', []],
  ['an Objective-C split', '@"Which team owns this?" @"billing" @"Parcels and delivery."\n@"My parcel went "\n    @"to the wrong address."', []],
  ['a C escape', '"{\\"choose\\":\\"Which team owns this?\\",\\"billing\\":\\"Parcels and delivery.\\"}" "My parcel went to the wrong address."', []],
  ['code between the pieces', 'q = "Which team owns this?" "billing" "Parcels and delivery."\nt = "My parcel went to the " + other + "wrong address."', ['My parcel went to the wrong address.']],
];

const failures = [];
for (const [name, text, expected] of CASES) {
  const got = missing(one, text);
  if (JSON.stringify(got) !== JSON.stringify(expected)) {
    failures.push(`${name}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(got)}`);
  }
}

if (failures.length) {
  console.error(`check-cases: ${failures.length} of ${CASES.length} cases fail\n  ${failures.join('\n  ')}`);
  process.exit(1);
}
console.log(`check-cases: all ${CASES.length} cases keep the rule.`);
