// The ABI boundary must reject codes that a compatible future engine may add.
// SQL cannot manufacture an engine call error, so this check uses its projector.
#include "../src/extension.cpp"
#include <climits>

int main() {
    ErrorOwner error;
    error.value.code = INT_MAX;
    try {
        Checked(THINKTHEN_ERROR, error);
    } catch (const InvalidInputException &failure) {
        return string(failure.what()).find("usage.abi") == string::npos ? 1 : 0;
    }
    return 1;
}
