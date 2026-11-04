using System.IO.Compression;
using System.Reflection.Metadata;
using System.Xml.Linq;

string feed = args[0];
using var package = ZipFile.OpenRead(Directory.GetFiles(feed, "*.nupkg").Single());
using var symbols = ZipFile.OpenRead(Directory.GetFiles(feed, "*.snupkg").Single());
using (var manifest = package.Entries.Single(x => x.FullName.EndsWith(".nuspec")).Open())
{
    var repository = XDocument.Load(manifest).Descendants().Single(x => x.Name.LocalName == "repository");
    if ((string?)repository.Attribute("url") != "https://github.com/botassembly/thinkthen"
        || repository.Attribute("commit") is not null)
        throw new Exception("Package advertises an unqualified source identity");
}
var pdbEntry = symbols.Entries.Single(x => x.FullName.EndsWith(".pdb"));
using var source = pdbEntry.Open();
using var copy = new MemoryStream();
source.CopyTo(copy); copy.Position = 0;
using var provider = MetadataReaderProvider.FromPortablePdbStream(copy);
var metadata = provider.GetMetadataReader();
if (metadata.Documents.Count == 0) throw new Exception("Symbols contain no source documents");
foreach (var document in metadata.Documents)
{
    string path = metadata.GetString(metadata.GetDocument(document).Name);
    if (!path.StartsWith("/_/", StringComparison.Ordinal))
        throw new Exception("Symbols expose a build-machine source path: " + path);
}
foreach (var handle in metadata.CustomDebugInformation)
{
    var entry = metadata.GetCustomDebugInformation(handle);
    if (metadata.GetGuid(entry.Kind) == new Guid("cc110556-a091-4d38-9fec-25ab9a351a6a"))
        throw new Exception("Private qualification symbols contain SourceLink");
}
Console.WriteLine("Portable symbols: mapped paths, no unqualified repository revision or source links");
