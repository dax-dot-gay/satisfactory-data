{
  pkgs,
  ...
}:
{
  languages = {
    rust.enable = true;
    dotnet = {
      enable = true;
      package = pkgs.dotnet-sdk_10;
    };
  };
  packages = [
    pkgs.git
    pkgs.cargo-autoinherit
    pkgs.openssl
    pkgs.steam-run
  ];
  scripts = {
    "aextract".exec = ''
        cd $(git rev-parse --show-toplevel)
        steam-run cargo run -p aextract -- $@
    '';
  };
}
