{
  inputs.flakelight-rust.url = "github:accelbread/flakelight-rust";
  outputs = { flakelight-rust, ... }:
    flakelight-rust ./. {
      systems = [ "aarch64-darwin" ];
    };
}
