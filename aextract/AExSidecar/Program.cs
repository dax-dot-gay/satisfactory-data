using CUE4Parse_Conversion;
using CUE4Parse_Conversion.Textures;
using CUE4Parse.Compression;
using CUE4Parse.FileProvider;
using CUE4Parse.MappingsProvider;
using CUE4Parse.UE4.Assets.Exports.Texture;
using CUE4Parse.UE4.Objects.Core.Serialization;
using CUE4Parse.UE4.Versions;
using Newtonsoft.Json;
using System.Text;

static class AExtract
{
    static void Main(string[] args)
    {
        string PaksDirectory = args[0];
        string CommunityDirectory = args[1];
        string Requests = args[2];
        string OodleLib = args[3];

        OodleHelper.Initialize(Path.GetFullPath(OodleLib));

        using StreamReader r = new(Path.Join(CommunityDirectory, "CustomVersions.json"));
        string json = Encoding.Unicode.GetString(Encoding.Default.GetBytes(r.ReadToEnd()));
        Console.WriteLine(json);
        List<FCustomVersion>? versions = JsonConvert.DeserializeObject<List<FCustomVersion>>(json);

        DirectoryInfo output_dir = System.IO.Directory.CreateDirectory("assets");
        var provider = new DefaultFileProvider(
            new DirectoryInfo(PaksDirectory),
            SearchOption.TopDirectoryOnly,
            new VersionContainer(
                game: EGame.GAME_UE5_3,
                customVersions: new FCustomVersionContainer(versions)
            )
        )
        {
            MappingsContainer = new FileUsmapTypeMappingsProvider(
                Path.Join(CommunityDirectory, "FactoryGame.usmap")
            ),
        };
        provider.Initialize();
        provider.Mount();
        provider.PostMount();

        using StreamReader reqs = new(Requests);
        foreach (var line in reqs.ReadToEnd().Split("\n"))
        {
            var ltrimmed = line.Trim();
            var parts = ltrimmed.Split("::", count: 3);
            if (parts.Length == 3)
            {
                var TargetId = parts[0];
                //var TargetType = parts[1];
                var TargetPath = parts[2];

                if (provider.TryLoadPackageObject(TargetPath, out var obj))
                {
                    switch (obj)
                    {
                        case UTexture2D when obj is UTexture2D texture:
                            var bmp = texture.Decode(ETexturePlatform.DesktopMobile);
                            if (bmp is null)
                                break;
                            var byt = bmp.Encode(ETextureFormat.Png, 9).ToArray();
                            var filename = Path.Combine("assets", $"{TargetId}.png");
                            File.WriteAllBytesAsync(filename, byt);
                            break;
                        default:
                            break;
                    }
                    Console.WriteLine($"Exported {TargetId}: {obj.ExportType}");
                }
            }
        }
    }
}