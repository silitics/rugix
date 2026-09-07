{ pkgs, mkBundle }:

let
  first = pkgs.writeText "first-payload" "first payload\n";
  second = pkgs.writeText "second-payload" "second payload\n";
  bundle = mkBundle {
    name = "test-update";
    version = "1";
    manifest = {
      update-type = "full";
      payloads = [
        {
          filename = "nested/first payload";
          delivery = {
            type = "slot";
            slot = "custom-system";
          };
          block-encoding = {
            chunker = "casync-64";
            deduplicate = true;
          };
        }
        {
          filename = "second";
          delivery = {
            type = "slot";
            slot = "custom-boot";
          };
        }
      ];
    };
    payloads = {
      "nested/first payload" = first;
      inherit second;
    };
  };
in
# Verify that bundling preserves multiple payloads, nested filenames, and delivery settings.
pkgs.runCommand "bundle-round-trip"
  {
    nativeBuildInputs = bundle.nativeBuildInputs ++ [ pkgs.python3 ];
  }
  ''
    test "$(cat ${bundle}/test-update.rugixb-hash)" = "$(rugix-bundler hash ${bundle}/test-update.rugixb)"
    rugix-bundler unpack ${bundle}/test-update.rugixb unpacked
    cmp ${first} 'unpacked/payloads/nested/first payload'
    cmp ${second} unpacked/payloads/second
    python3 - <<'PY'
    import tomllib
    with open("${bundle.manifestFile}", "rb") as source:
        expected = tomllib.load(source)
    with open("unpacked/rugix-bundle.toml", "rb") as source:
        actual = tomllib.load(source)
    assert actual == expected, (actual, expected)
    PY
    touch "$out"
  ''
