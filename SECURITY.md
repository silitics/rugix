# Security Policy

> [!CAUTION]
> **Do not report security vulnerabilities through GitHub issues or any other public medium.**

We take the security of Rugix and our users very seriously.
If you have found a potential security vulnerability, please report it according to the process outlined in the *Responsible Disclosure* section below.
Once a security vulnerability has been confirmed and fixed, we publish a *GitHub Security Advisory* describing the issue, the affected versions, and the necessary mitigations.


## Supported Versions

Rugix follows a rolling support model focused on the most recent stable release series.
Only the latest minor version branch receives security updates.

| Version | Supported          |
| ------- | ------------------ |
| 0.8.x   | :white_check_mark: |
| 0.7.x   | :x:                |
| 0.6.x   | :x:                |

If you are using an unsupported version, please upgrade to the latest release series to continue receiving security fixes.


## Responsible Disclosure

If you have found a potentially vulnerability in Rugix or any related tooling and dependencies, please send an email to **security@silitics.com** OR use use [GitHub's responsible disclosure functionality](https://github.com/silitics/rugix/security/advisories/new).
Please include a detailed technical description in your report, such that we can validate and asses the potential vulnerability.

### What to Expect

- **Acknowledgement:** We will confirm receipt of your report within **3 business days**.
- **Initial Assessment:** We aim to provide an initial assessment within **7 business days**.
- **Coordinated Fix:** If the vulnerability is validated, we will work with you on a coordinated disclosure timeline and keep you informed about progress.
- **Disclosure:** After a fix is released, we will publicly document the vulnerability, crediting you (if you wish) for the discovery.

### Encrypted Emails

Consider encrypting your email using the following PGP key:

```
-----BEGIN PGP PUBLIC KEY BLOCK-----
mDMEaR4VsRYJKwYBBAHaRw8BAQdALx1EaWIe3n4AKaZ5u6atvBjmMjjM1RV0thXy
/sqbJUe0LlNpbGl0aWNzIFNlY3VyaXR5IFRlYW0gPHNlY3VyaXR5QHNpbGl0aWNz
LmNvbT6ImQQTFgoAQRYhBFCEE5HqPt3CIZUDaZl4a0gzQ9lbBQJpHhWxAhsDBQkJ
ZgGABQsJCAcCAiICBhUKCQgLAgQWAgMBAh4HAheAAAoJEJl4a0gzQ9lb96gBAJTK
H21kmqJA+WkNX8OFYWwbIInEhfRtw29zzdjRqrWXAQD70KCHtKV/mUKSBDsaJXQU
5NQEKPS346a5NBtiktq0BLg4BGkeFbESCisGAQQBl1UBBQEBB0CJrcWCzo5V6y97
rjhyXcCu4nxEWCJNRX3idiqD8xRdCwMBCAeIfgQYFgoAJhYhBFCEE5HqPt3CIZUD
aZl4a0gzQ9lbBQJpHhWxAhsMBQkJZgGAAAoJEJl4a0gzQ9lb/IoA/3Rn4QlQeefj
W39VCTdBXsNajoiZLYo72NYAePO2gB9sAP4zd2psZxjySrTMYr9sFj4WmjqnZx31
EiF9t/Qb2fAnBg==
=ILCx
-----END PGP PUBLIC KEY BLOCK-----
```

Fingerprint: `50841391EA3EDDC22195036999786B483343D95B`

### Non-Security Issues

For general bugs or feature requests, please use the standard GitHub issue tracker instead of the security contact.

---

Thank you for helping keep Rugix and its users safe.
