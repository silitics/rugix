---
sidebar_position: 6
---

# Signed Updates

When updates are installed from untrusted sources, such as user-provided update bundles, it is important to verify that updates are genuine and have not been tampered with.
To this end, Rugix Ctrl supports *embedded signatures* in update bundles.
Embedded signatures are based on the widely-adopted [*Cryptographic Message Syntax* (CMS) standard](https://datatracker.ietf.org/doc/html/rfc5652), which supports certificate-based signature verification.
This allows existing CA infrastructure to be reused for signing and verifying update bundles.
Furthermore, updates can be signed using the PKCS#11 interface, which allows for secure key storage in *Hardware Security Modules* (HSMs).


## Signing Bundles

To sign a bundle, you need a certificate and a private key.
You can then use the following command to sign a bundle:

```shell
rugix-bundler signatures sign <BUNDLE> <CERT> <KEY> <OUT>
```

Additional intermediate certificates can be included using the `--intermediate-cert` option.

If you do not have a certificate and private key, you can follow the following steps to create a simple self-signed CA:

1. Generate a root certificate and private key:
    ```shell
    openssl ecparam -name prime256v1 -genkey -noout -out root.key
    openssl req -x509 -new -key root.key -out root.crt -days 3650 -subj "/CN=Update CA"
    ```
2. Generate a short-lived certificate and private key for signing update bundles:
    ```shell
    openssl ecparam -name prime256v1 -genkey -noout -out signer.key
    openssl req -new -key signer.key -out signer.csr -subj "/CN=Update Signer"
    openssl x509 -req -in signer.csr \
        -CA root.crt -CAkey root.key -CAcreateserial \
        -out signer.crt -days 365
    ```

The root certificate is valid for 10 years and should be deployed to the devices for verifying updates.
The signer certificate is valid for one year and should be used for signing bundles.
**Private keys must be kept secret!**


### External Signing

Instead of signing a bundle directly through `rugix-bundler`, you can also create a raw *signed metadata* file and sign it through some external means.
To generate a signed metadata file, use the following command:

```shell
rugix-bundler signatures prepare <BUNDLE> signed-metadata.raw
```

This will create a file `signed-metadata.raw`, which can then be signed externally, for instance, using `openssl cms`:

```shell
openssl cms -sign \
  -in signed-metadata.raw \
  -signer signer.crt -inkey signer.key \
  -out signed-metadata.cms -outform DER \
  -nosmimecap -nodetach -binary
```

Note that the resulting CMS signature file must be in DER format and must include the signed content.

The externally created CMS signature file can then be added to a bundle using the following command:

```shell
rugix-bundler signatures add <BUNDLE> signed-metadata.cms <OUT>
```

This allows any external tool to be used to create a CMS signature for a bundle and then add it to the bundle.
In particular, this allows PKCS#11-compatible HSMs to be used for signing through `openssl cms`.

:::warning

**While currently not enforced, Rugix Ctrl only supports CMS signatures with a single signer.**

:::


## Verifying Bundles

To verify that a bundle has been signed by a root of trust, you can use the following command:

```shell
rugix-bundler signatures verify <BUNDLE> <CERT> <OUT>
```

Note that the certificate does not need to be the certificate used for signing but can be any certificate serving as a root of trust for which a certificate chain can be established using the certificates embedded in the CMS signature.

To enforce signatures when installing updates through Rugix Ctrl, use the following options:

```
--verify-signature --root-cert <CERT>
```

:::danger

**Signature verification must be explicitly enabled.**
In the future, we will make signature verification the mandatory default.

:::
