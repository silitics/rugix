{
  pkgs,
  mkComposeBundle,
}:

let
  image = pkgs.dockerTools.buildLayeredImage {
    name = "localhost/test-image";
    tag = "original";
    contents = [ pkgs.busybox ];
    config.Cmd = [ "/bin/true" ];
  };
  bundle = mkComposeBundle {
    name = "test-app";
    version = "1";
    compose.services.worker.image = "localhost/test-image:original";
    images.worker = image;
  };
in
# Use real archives and Skopeo to verify payloads, content tags, and archive-source pinning.
pkgs.runCommand "compose-bundle-test"
  {
    nativeBuildInputs = bundle.nativeBuildInputs ++ [ pkgs.jq ];
  }
  ''
    test "$(cat ${bundle}/test-app.rugixb-hash)" = "$(rugix-bundler hash ${bundle}/test-app.rugixb)"
    rugix-bundler unpack ${bundle}/test-app.rugixb unpacked
    mkdir base
    tar -xf unpacked/payloads/base.tar -C base
    tag=$(jq -r '.images[0].bundleTag' base/images/rugix-images.json)
    case "$tag" in
      localhost/rugix-apps/test-app/image-0:m-*) ;;
      *) echo "Expected a bundle-local content tag" >&2; exit 1 ;;
    esac
    grep -F "$tag" base/docker-compose.yml
    grep -F 'pull_policy: never' base/docker-compose.yml
    if grep -q x-rugix base/docker-compose.yml; then
      echo "Build-only image options leaked into packaged Compose" >&2
      exit 1
    fi
    jq -e '.images[0].source == "docker-archive" and (.images[0].sourceDigest | startswith("sha256:"))' base/images/rugix-images.json
    skopeo --tmpdir "$TMPDIR" inspect "docker-archive:$PWD/unpacked/payloads/image-0.tar:$tag" > image.json
    tar -xOf unpacked/payloads/image-0.tar manifest.json \
      | jq -e --arg tag "$tag" '.[0].RepoTags | index($tag) != null'

    mkdir relative
    ln -s ${image} relative/image.tar.gz
    cat > relative/compose.yml <<EOF_COMPOSE
    services:
      worker:
        image: localhost/test-image:original
        x-rugix:
          image:
            source: docker-archive
            ref: image.tar.gz
    EOF_COMPOSE
    rugix-bundler apps pack docker-compose --app test-app --disable-pinning relative/compose.yml "$PWD/unpinned.rugixb"
    rugix-bundler unpack unpinned.rugixb unpinned
    skopeo --tmpdir "$TMPDIR" inspect "docker-archive:$PWD/unpinned/payloads/image-0.tar:localhost/test-image:original" > /dev/null

    rm relative/image.tar.gz
    if rugix-bundler apps pack docker-compose --app test-app relative/compose.yml "$PWD/missing.rugixb"; then
      echo "Bundler accepted a missing archive" >&2
      exit 1
    fi
    test ! -e missing.rugixb
    touch "$out"
  ''
