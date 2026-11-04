import { RecordError, ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  await tt.decide("Does the customer request a refund?", "");
} catch (error) {
  if (!(error instanceof RecordError)) throw error;
  console.log(error.code);
} finally {
  await tt.close();
}
