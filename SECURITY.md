# Security Policy

We take the security of Rugix very seriously and aim to keep our users informed about resolved vulnerabilities. When a security issue is fixed, we publish a **GitHub Security Advisory** describing the issue, the affected versions, and necessary mitigations. Where appropriate, we request or assign a **CVE ID** for tracking.

**Do not report security vulnerabilities through GitHub issues or any other public medium.** See the section on *Responsible Disclosure* below for reporting security vulnerabilities in Rugix.


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

If you believe you have found a vulnerability in Rugix or any related tooling, please send an email to **security@silitics.com**. The email should include a detailed technical description of the potential vulnerability such that we can validate it.

### What to Expect

- **Acknowledgement:** We will confirm receipt of your report within **3 business days**.
- **Initial Assessment:** We aim to provide an initial assessment within **7 business days**.
- **Coordinated Fix:** If the issue is validated, we will work with you on a coordinated disclosure timeline and keep you informed about progress.
- **Disclosure:** After a fix is released, we will publicly document the vulnerability, crediting you (if you wish) for the discovery.

### Encrypted Disclosure

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
