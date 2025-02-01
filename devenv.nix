{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
      pkgs.curl
      pkgs.git
      pkgs.jq
      pkgs.rustup
      pkgs.sccache
      pkgs.cargo-outdated
      pkgs.cargo-nextest
      pkgs.cargo-audit
      pkgs.just
      pkgs.tree
      pkgs.werf
      pkgs.kubernetes-helm
      pkgs.vault-medusa
      # pkgs.vault-bin
      # pkgs.ngrok
  ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
       frameworks.SystemConfiguration
       frameworks.Security
       frameworks.CoreFoundation
     ]);

  # https://devenv.sh/languages/
  languages.nix.enable = true;
  languages.rust.enable = true;

  env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  # languages.rust.mold.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  services.clickhouse.enable = true;
  services.clickhouse.httpPort = 8111;
  services.clickhouse.port = 9111;

  
  # environment.etc = {
  #   # With changes from https://theorangeone.net/posts/calming-down-clickhouse/
  #   "clickhouse-server/config.d/custom.xml".source = lib.mkForce ./clickhouse-config.xml;
  #   "clickhouse-server/users.d/custom.xml".source = lib.mkForce ./clickhouse-users.xml;
  # };

  env.CLICKHOUSE_ADDR = "127.0.0.1:9111";
}
