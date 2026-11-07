// Stand in for the runtime's report: the named glibc, or none as on musl.
const glibc = process.env.THINKTHEN_TEST_GLIBC;
process.report.getReport = () => ({header: glibc ? {glibcVersionRuntime: glibc} : {}});
