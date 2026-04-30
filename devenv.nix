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
  ];
}
