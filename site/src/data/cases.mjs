// The one case each function's samples show: a customer-support inbox.
// Every language and database asks the same question, with the same
// choices, of the same input. scripts/check-cases.mjs fails the build when
// a function's first Bash example, a language or SQL sample, or a question
// file it reads lacks one of these strings.
//
// question: the question, or each question of a saved form.
// choices: the options, labels, levels, kinds or relation parts.
// input: the input text, in pieces no sample splits.

export const CASES = {
  decide: {
    question: ['Does the customer ask for a refund?'],
    choices: [],
    input: ['Please refund my order.', 'It arrived broken.'],
  },
  choose: {
    question: ['Which team owns this?'],
    choices: [
      'billing', 'Invoices, fees, and refunds.',
      'shipping', 'Parcels and delivery.',
      'account', 'Logins and passwords.',
    ],
    input: ['My parcel went to the wrong address.'],
  },
  tag: {
    question: ['Which labels fit this message?'],
    choices: ['praise', 'bug', 'billing'],
    input: ['Love the new dashboard,', 'but export crashes the app,', 'and I was charged twice.'],
  },
  score: {
    question: ['How urgent is this?'],
    choices: ['Routine.', 'Soon.', 'Immediate.'],
    input: ['Our checkout page is down and customers cannot pay.'],
  },
  filter: {
    question: ['Is this a complaint?'],
    choices: [],
    input: [
      'Arrived a day early. Thank you!',
      'The zipper broke the first time I used it.',
      'Does this come in blue?',
      'The strap snapped on day two.',
    ],
  },
  rank: {
    question: ['Is this urgent?'],
    choices: [],
    input: [
      'Newsletter: our autumn catalog is here.',
      'No reply needed.',
      'Our checkout page is down and customers cannot pay',
      'Reminder: your invoice is due in 30 days',
      'Please send the signed quote by 5 pm today',
    ],
  },
  find: {
    question: ['Which line gives the refund deadline?'],
    choices: [],
    input: [
      'Returns need the original receipt.',
      'Refunds are issued within 30 days of purchase.',
      'Shipping is free on orders over $50.',
      'Gift cards cannot be exchanged for cash.',
    ],
  },
  annotate: {
    question: [
      'Does the report give steps to reproduce?',
      'Which part of the app is this?',
      'How much does this block the user?',
    ],
    choices: ['export', 'login', 'billing', 'None.', 'Slows them.', 'Blocks work.'],
    input: ['Steps: click Log in.', 'Nobody gets in.'],
  },
  recognize: {
    question: [],
    choices: ['person', 'product', 'place'],
    input: ['Sam Patel says his Kindle arrived broken', 'in Denver.'],
  },
  relate: {
    question: [],
    choices: ['makes', 'company', 'product'],
    input: ['Apple', 'Amazon', 'iPhone', 'Kindle'],
  },
};
