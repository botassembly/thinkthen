import thinkthen as tt

try:
    tt.decide("Does the customer request a refund?", "")
except tt.RecordError as failure:
    print(failure.code)
