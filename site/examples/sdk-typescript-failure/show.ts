import { FailedError, Client } from "thinkthen";

const tt = new Client();
try {
  await tt.decide("Does the customer request a refund?", "");
} catch (error) {
  if (!(error instanceof FailedError)) throw error;
  console.log(error.code);
} finally {
  await tt.close();
}
