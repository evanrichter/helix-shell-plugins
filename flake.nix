{
  inputs.flakelight-rust.url = "github:accelbread/flakelight-rust";
  outputs = { flakelight-rust, ... }:
    flakelight-rust ./. {
      systems = [ "aarch64-darwin" "x86_64-linux" ];
    };
}
