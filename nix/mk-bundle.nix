{
  lib,
  runCommand,
  formats,
  rugixBundler,
}:

# The manifest uses Rugix's existing Sidex-defined bundle format.
{
  name,
  version,
  manifest,
  payloads,
}:

assert lib.assertMsg (
  lib.sort builtins.lessThan (map (payload: payload.filename) manifest.payloads)
  == builtins.attrNames payloads
) "mkBundle requires exactly one source for each manifest payload filename";
assert lib.assertMsg (lib.all
  (filename: lib.all (part: part != "" && part != "." && part != "..") (lib.splitString "/" filename))
  (builtins.attrNames payloads)
) "mkBundle payload filenames must be relative paths without empty, dot, or parent segments";

let
  manifestFile = (formats.toml { }).generate "${name}-manifest.toml" manifest;
  stagePayloads = lib.concatStringsSep "\n" (
    lib.mapAttrsToList (filename: source: ''
      mkdir -p -- ${lib.escapeShellArg "bundle/payloads/${builtins.dirOf filename}"}
      cp -- ${lib.escapeShellArg (toString source)} ${lib.escapeShellArg "bundle/payloads/${filename}"}
    '') payloads
  );
in
runCommand "${name}-${version}"
  {
    nativeBuildInputs = [ rugixBundler ];
    passthru = { inherit manifestFile payloads; };
  }
  ''
    mkdir -p bundle/payloads "$out"
    cp ${manifestFile} bundle/rugix-bundle.toml
    ${stagePayloads}
    rugix-bundler bundle bundle "$out"/${lib.escapeShellArg "${name}.rugixb"}
    rugix-bundler hash "$out"/${lib.escapeShellArg "${name}.rugixb"} \
      > "$out"/${lib.escapeShellArg "${name}.rugixb-hash"}
  ''
