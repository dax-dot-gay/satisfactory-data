{
  pkgs,
  ...
}:

{
  languages = {
    rust.enable = true;
    dotnet.enable = true;
  };
  packages = [
    pkgs.git
    pkgs.cargo-autoinherit
  ];
}
