@SuppressWarnings("module")
module dev.thinkthen {
    requires com.fasterxml.jackson.databind;
    requires static transitive org.jspecify;
    exports dev.thinkthen;
    exports dev.thinkthen.internal to dev.thinkthen.kotlin;
}
