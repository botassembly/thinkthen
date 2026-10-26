@SuppressWarnings("module")
module io.thinkthen {
    requires com.fasterxml.jackson.databind;
    requires static transitive org.jspecify;
    exports io.thinkthen;
    exports io.thinkthen.internal to io.thinkthen.kotlin;
}
