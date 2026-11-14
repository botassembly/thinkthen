import thinkthen as tt

try:
    tt.decide("Does the customer request a refund?", "").raise_if_failed()
except tt.Failed as failure:
    print(failure.code)
